//! Optional passive borrowed boundaries of the genuine persistent CTF owner.
//! No collection limit, owner clone, math, clock reconstruction or runtime input.

use super::ctf_public_initialization::OrderedCtfPublicInitialization;
use super::ctf_surface_binding::{CtfSurfaceGenerationContext, CtfSurfaceGenerationKey};
use super::ctf_surface_lifecycle::CtfSurfaceLifecycleUnavailable;
use super::ctf_thermal_history::{
    CtfThermalHistoryContext, CtfThermalHistoryState, CtfThermalHistorySurface,
    CtfThermalHistoryUpdateRoute,
};
use super::state::SurfaceHeatBalanceState;
use crate::weather::day::ProductionCtfHistoryClock;
use ep_model::{SurfaceId, TypedModel};
use std::cell::RefCell;

/// Precise Rust boundaries; none asserts a Native callback correspondence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CtfHistoryBoundary {
    /// Before the real begin wrapper's checks or any stores.
    BeginEntry,
    /// After genuine bulk report reset, immediately before selected history Init.
    InitializationInput,
    /// Selected Init returned; lifecycle initialized/current/lagged stores follow.
    InitializationReturned,
    /// Real begin returned after lagged copies and current clock acceptance.
    BeginReturned,
    /// Real begin failed; the first sticky failure is already retained.
    BeginFailed,
    /// Actual current surface temperatures have been stored in normal term1.
    HistoryUpdateInput,
    /// Selected update returned its real route; lagged consumer copies follow.
    HistoryUpdateReturned,
    /// Copies and last-committed store completed; current clock is still owned.
    HistoryCommitBeforeClockRelease,
    /// Actual commit returned with its current clock released.
    CommitReturned,
    /// Actual commit failed; no successful return is inferred.
    CommitFailed,
}

/// Borrow the exact private persistent owners, including uninitialized allocation.
#[derive(Clone, Copy, Debug)]
pub struct CtfHistoryOwnerView<'a> {
    /// Exact producer-retained equality key; never a Native ID or lossy digest.
    pub generation_key: &'a CtfSurfaceGenerationKey,
    /// Actual original producer inputs and compiled witnesses.
    pub generation_context: CtfSurfaceGenerationContext,
    /// Full normal19/master/counter/scratch/report owners, including all columns.
    pub history: &'a CtfThermalHistoryState,
    /// Actual independent Zone/Space traversal groups.
    pub heat_transfer_groups: &'a [Vec<SurfaceId>],
    /// Actual independent opaque traversal groups.
    pub opaque_groups: &'a [Vec<SurfaceId>],
    /// Actual separate AllHT order.
    pub all_heat_transfer_surfaces: &'a [SurfaceId],
    /// Actual lifecycle guard, not inferred from temperature values.
    pub initialized: bool,
    /// Actual accepted current clock, possibly absent at entry/failure/return.
    pub current_clock: Option<ProductionCtfHistoryClock>,
    /// Actual previous successful commit identity.
    pub last_committed_clock: Option<ProductionCtfHistoryClock>,
    /// First actual retained ownership failure.
    pub failure: Option<CtfSurfaceLifecycleUnavailable>,
}

/// Everything is borrowed from this genuine call; the sink cannot mutate it.
#[derive(Clone, Copy, Debug)]
pub struct CtfHistoryObservation<'a> {
    /// Precise current Rust boundary.
    pub boundary: CtfHistoryBoundary,
    /// Actual declared identity/name/Zone/Space/construction input owner.
    pub model: &'a TypedModel,
    /// Actual moved public owner; absent remains absent on failure.
    pub public: Option<&'a OrderedCtfPublicInitialization>,
    /// Actual persistent owner view.
    pub owner: CtfHistoryOwnerView<'a>,
    /// Actual full consumer storage, with current and all active lagged values.
    pub consumer_surfaces: &'a [SurfaceHeatBalanceState],
    /// Actual caller argument, independent of the owner's accepted clock.
    pub caller_clock: Option<ProductionCtfHistoryClock>,
    /// Actual independent begin-wrapper timestep argument; absent at other seams.
    pub begin_timestep_seconds_argument: Option<f64>,
    /// Actual outdoor scalar passed to this call; not a fabricated weather owner.
    pub outdoor_dry_bulb_c: f64,
    /// Actual prepared selected operands only at selected-function boundaries.
    pub selected_inputs: Option<&'a [CtfThermalHistorySurface<'a>]>,
    /// Actual passed context at those same boundaries.
    pub selected_context: Option<CtfThermalHistoryContext<'a>>,
    /// Actual update return value only after a successful selected update.
    pub update_route: Option<CtfThermalHistoryUpdateRoute>,
    /// Actual wrapper error; retained failure may be an earlier error.
    pub error: Option<CtfSurfaceLifecycleUnavailable>,
}

type Sink = Box<dyn for<'a> FnMut(CtfHistoryObservation<'a>)>;
thread_local! {
    static ACTIVE: RefCell<Option<Sink>> = const { RefCell::new(None) };
}

struct Guard(Option<Sink>);
impl Drop for Guard {
    fn drop(&mut self) {
        ACTIVE.with_borrow_mut(|active| *active = self.0.take());
    }
}

/// Execute the same caller with an optional thread-local passive sink. Disabled
/// capture does not box/clone any owner. Nested calls and unwinding restore the
/// preceding sink. A callback must remain passive and non-reentrant; retain I/O
/// failures outside the solver and report incomplete capture after it returns.
pub fn with_sink<R>(
    enabled: bool,
    sink: impl for<'a> FnMut(CtfHistoryObservation<'a>) + 'static,
    execute: impl FnOnce() -> R,
) -> R {
    if !enabled {
        return execute();
    }
    let previous = ACTIVE.with_borrow_mut(|active| active.replace(Box::new(sink)));
    let _guard = Guard(previous);
    execute()
}

/// Allows the existing caller to avoid even constructing optional views off path.
#[must_use]
pub fn is_active() -> bool {
    ACTIVE.with_borrow(Option::is_some)
}

pub(crate) fn record(observation: CtfHistoryObservation<'_>) {
    ACTIVE.with_borrow_mut(|active| {
        if let Some(sink) = active {
            sink(observation);
        }
    });
}

#[cfg(test)]
#[path = "ctf_history_capture_tests.rs"]
mod tests;
