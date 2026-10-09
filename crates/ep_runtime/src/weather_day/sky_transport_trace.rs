//! Passive scalar receipts for actual owned sky transport on the collecting thread.

use super::WeatherDayPhase;
use ep_model::SurfaceId;
use std::{cell::RefCell, collections::BTreeMap, panic::Location};

/// Predeclared retained prefix per event kind; totals include every omitted call.
pub const EVENT_LIMIT_PER_KIND: usize = 1_000_000;
/// Seven transport roles, with hourly vector output and series handoff distinct.
pub const EVENT_KIND_COUNT: usize = 8;

/// Copy identity captured immediately after the actual Current completion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SkyTransportStamp {
    /// Actual production phase, including independent warmup days.
    pub phase: WeatherDayPhase,
    /// Actual thermal caller's civil hourly index, not a native record ordinal.
    pub record_index: usize,
    /// Actual one-based global hour at completion.
    pub hour: i32,
    /// Actual one-based global zone timestep at completion.
    pub timestep: i32,
    /// All selected weather completions, including nested calls, before receipt.
    pub completed_operation_count: u64,
}

/// Stable event kinds; repeated calls and repeated step identities are retained.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SkyTransportKind {
    /// Actual Today/context/sample receipt.
    OwnedContextReceipt,
    /// Actual sampler locals immediately before return.
    SamplerReturn,
    /// Actual operands and accumulator values before/after existing additions.
    ReportAccumulator,
    /// Actual just-pushed hourly values and divisor.
    HourlyOutput,
    /// Actual constructed OutputSeries values at handoff.
    SeriesHandoff,
    /// Actual physical function arguments, before its early return.
    PhysicalIngress,
    /// Actual resolved local immediately before physical longwave invocation.
    PhysicalResolved,
    /// Separate actual report recomputation longwave argument.
    ReportResolved,
}

impl SkyTransportKind {
    /// Stable bucket order, separate from the actual global event sequence.
    #[must_use]
    pub const fn index(self) -> usize {
        self as usize
    }

    /// Stable output identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::OwnedContextReceipt => "owned_context_receipt",
            Self::SamplerReturn => "run_period_weather_sampler_return",
            Self::ReportAccumulator => "run_period_report_accumulator",
            Self::HourlyOutput => "run_period_report_hourly_output",
            Self::SeriesHandoff => "run_period_report_series_handoff",
            Self::PhysicalIngress => "physical_exterior_balance_ingress",
            Self::PhysicalResolved => "physical_resolved_longwave_argument",
            Self::ReportResolved => "report_recomputed_longwave_argument",
        }
    }
}

/// Explicit upstream origin; never inferred from mutable TLS or event timing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SkyTransportOrigin {
    /// Existing default boundary wrapper used by actual surface solves.
    SurfaceSolve,
    /// Actual outside-face report recomputation sharing the same kernel.
    ReportOutsideFace,
    /// Direct kernel callers/tests; no timestep-solve origin is asserted.
    ExplicitDirect,
    /// Separate surface_exterior_report_terms longwave recomputation.
    ReportTerms,
}

impl SkyTransportOrigin {
    /// Passive caller-route identifier, separate from actual event kind.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::SurfaceSolve => "surface_solve",
            Self::ReportOutsideFace => "report_outside_face_recomputation",
            Self::ExplicitDirect => "explicit_direct_call",
            Self::ReportTerms => "report_terms_recomputation",
        }
    }
}

/// Actual output series identity; no expected hourly values are calculated.
#[derive(Clone, Copy, Debug)]
pub enum SkyTransportSeries {
    /// Site Sky Temperature / C.
    SkyTemperature,
    /// Site Horizontal Infrared Radiation Rate per Area / W/m2.
    HorizontalInfrared,
}

/// Small actual scalar payloads; no session, grid, or model is cloned per call.
#[derive(Clone, Copy, Debug)]
pub enum SkyTransportValues {
    /// Actual Today four-field owner and the actual sample IR field.
    OwnedContext {
        /// SkyTemp, HorizIRSky, TotalSkyCover, OpaqueSkyCover, in that order.
        today: [f64; 4],
        /// Actual sample horizontal infrared value after its owner assignment.
        sample_ir: f64,
    },
    /// Actual sampler locals; absent owned context remains explicit.
    Sampler {
        /// Actual returned SkyTemp and IR locals, respectively.
        values: [f64; 2],
        /// Whether these locals were read from an actual owned context.
        owned: bool,
    },
    /// Existing additions are observed without reordering or reproducing them.
    Accumulator {
        /// Actual reporting hour index.
        hour_index: usize,
        /// Actual substep loop index.
        substep: u32,
        /// Received SkyTemp and IR locals.
        received: [f64; 2],
        /// Actual accumulator values immediately before the additions.
        before: [f64; 2],
        /// Actual accumulator values immediately after the additions.
        after: [f64; 2],
    },
    /// Actual hourly vector writes, not reconstructed expected means.
    Hourly {
        /// Actual reporting hour index.
        hour_index: usize,
        /// Actual divisor used by the existing vector writes.
        divisor: f64,
        /// Actual just-pushed SkyTemp and IR values.
        pushed: [f64; 2],
    },
    /// Actual value borrowed from the constructed series before moving it.
    Series {
        /// Actual vector sample index.
        hour_index: usize,
        /// Actual OutputHandle value.
        handle: u32,
        /// Actual series role.
        series: SkyTransportSeries,
        /// Actual stored value, without a second averaging calculation.
        value: f64,
    },
    /// Actual physical ingress, preserving the Option rather than filling it.
    Ingress {
        /// Explicit actual upstream caller route.
        origin: SkyTransportOrigin,
        /// Actual optional sky argument.
        sky: Option<f64>,
        /// Actual IR argument.
        ir: f64,
    },
    /// Actual resolved local from a physical or report call, when it executes.
    Resolved {
        /// Explicit actual upstream caller route.
        origin: SkyTransportOrigin,
        /// Actual resolved sky argument.
        sky: f64,
        /// Actual IR local/argument used by this path.
        ir: f64,
        /// True only when the actual sky source was the owned Some path.
        owned: bool,
    },
}

/// One actual hook occurrence. Missing identities are never filled from TLS.
#[derive(Clone, Copy, Debug)]
pub struct SkyTransportEvent {
    /// Actual global order, including occurrences omitted from other buckets.
    pub sequence: u64,
    /// Actual occurrence order within this kind.
    pub kind_sequence: u64,
    /// Actual hook role.
    pub kind: SkyTransportKind,
    /// Copy identity transported with the actual operand, or unavailable.
    pub stamp: Option<SkyTransportStamp>,
    /// Actual surface identity for surface hooks only.
    pub surface_id: Option<SurfaceId>,
    /// Actual hook source location.
    pub caller: &'static Location<'static>,
    /// Actual observed scalars and discrete identities.
    pub values: SkyTransportValues,
}

/// Owned trace. Empty Vecs allocate only as real retained occurrences arrive.
#[derive(Debug, Default)]
pub struct SkyTransportTrace {
    /// All actual weather operation completions while this collector is active.
    pub completed_operation_count: u64,
    /// All actual hook occurrences; no identity deduplication.
    pub total_event_count: u64,
    /// All occurrence counts, including omitted suffixes, in stable kind order.
    pub total_by_kind: [u64; EVENT_KIND_COUNT],
    /// Independent contiguous prefixes. Global sequence permits an ordered merge.
    pub retained_by_kind: [Vec<SkyTransportEvent>; EVENT_KIND_COUNT],
    /// Actual surface names copied once per retained surface id.
    pub surface_names: BTreeMap<SurfaceId, String>,
}

thread_local! { static ACTIVE: RefCell<Option<SkyTransportTrace>> = RefCell::default(); }
struct Guard(Option<SkyTransportTrace>);
impl Drop for Guard {
    fn drop(&mut self) {
        ACTIVE.with_borrow_mut(|active| *active = self.0.take());
    }
}

/// Captures only this thread; nesting and unwinding restore the earlier owner.
pub fn capture<R>(enabled: bool, execute: impl FnOnce() -> R) -> (R, Option<SkyTransportTrace>) {
    if !enabled {
        return (execute(), None);
    }
    let previous = ACTIVE.with_borrow_mut(|active| active.replace(Default::default()));
    let _guard = Guard(previous);
    let result = execute();
    (result, ACTIVE.with_borrow_mut(Option::take))
}

/// Active capture on this thread; off-mode hourly stamp storage stays empty.
#[must_use]
pub(crate) fn is_active() -> bool {
    ACTIVE.with_borrow(|active| active.is_some())
}

/// Called at the actual selected completion hook before the old Full-only guard.
pub(super) fn operation_completed() {
    ACTIVE.with_borrow_mut(|active| {
        if let Some(trace) = active {
            trace.completed_operation_count += 1;
        }
    });
}

/// Captures an actual completion prefix once, after the real Current call.
pub(super) fn completed_stamp(
    phase: WeatherDayPhase,
    record_index: usize,
    hour: i32,
    timestep: i32,
) -> Option<SkyTransportStamp> {
    ACTIVE.with_borrow(|active| {
        active.as_ref().map(|trace| SkyTransportStamp {
            phase,
            record_index,
            hour,
            timestep,
            completed_operation_count: trace.completed_operation_count,
        })
    })
}

/// Records actual scalars only. Missing identities and fallback paths stay honest.
#[track_caller]
pub(crate) fn record(
    kind: SkyTransportKind,
    stamp: Option<SkyTransportStamp>,
    values: SkyTransportValues,
    surface: Option<(SurfaceId, &str)>,
) {
    let caller = Location::caller();
    ACTIVE.with_borrow_mut(|active| {
        let Some(trace) = active else {
            return;
        };
        let index = kind.index();
        trace.total_event_count += 1;
        trace.total_by_kind[index] += 1;
        if trace.retained_by_kind[index].len() == EVENT_LIMIT_PER_KIND {
            return;
        }
        if let Some((id, name)) = surface {
            trace
                .surface_names
                .entry(id)
                .or_insert_with(|| name.to_owned());
        }
        trace.retained_by_kind[index].push(SkyTransportEvent {
            sequence: trace.total_event_count,
            kind_sequence: trace.total_by_kind[index],
            kind,
            stamp,
            surface_id: surface.map(|(id, _)| id),
            caller,
            values,
        });
    });
}

/// Borrows each actual constructed series value immediately before its existing move.
/// Track-caller propagation retains the reports.rs handoff site in every receipt.
#[track_caller]
pub(crate) fn record_series_handoff(
    output: &crate::OutputSeries,
    stamps: &[Option<SkyTransportStamp>],
    series: SkyTransportSeries,
) {
    if !is_active() {
        return;
    }
    for (hour_index, &value) in output.values.iter().enumerate() {
        record(
            SkyTransportKind::SeriesHandoff,
            stamps.get(hour_index).copied().flatten(),
            SkyTransportValues::Series {
                hour_index,
                handle: output.handle.0,
                series,
                value,
            },
            None,
        );
    }
}

/// Borrows actual pushed hourly values and preserves the trace-only stamp cap.
/// The caller retains its capture guard; no hourly scalar or mean is recalculated.
#[track_caller]
pub(crate) fn record_hourly_output(
    sky_values: &[f64],
    infrared_values: &[f64],
    stamps: &mut Vec<Option<SkyTransportStamp>>,
    stamp: Option<SkyTransportStamp>,
    hour_index: usize,
    divisor: f64,
) {
    if stamps.len() < EVENT_LIMIT_PER_KIND {
        stamps.push(stamp);
    }
    if let (Some(&sky), Some(&ir)) = (sky_values.last(), infrared_values.last()) {
        record(
            SkyTransportKind::HourlyOutput,
            stamp,
            SkyTransportValues::Hourly {
                hour_index,
                divisor,
                pushed: [sky, ir],
            },
            None,
        );
    }
}
