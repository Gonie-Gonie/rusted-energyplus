//! Passive copies of schedule caches at explicit production ownership boundaries.

use super::ScheduleSeriesCache;
use std::cell::RefCell;

/// The actual callsite supplies this origin; no latest-context inference is used.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScheduleCacheOrigin {
    /// The pipeline's hourly cache, prepared for both A and B.
    PipelineHourlyPrepared,
    /// The pipeline's zone-timestep environment cache, prepared for B.
    PipelineEnvironmentPrepared,
    /// The actual cache argument received by the public B production entry.
    CoupledProductionArgument,
    /// A's separate referenced-only cache, just constructed for initialization.
    HeatBalanceReferencedInitialization,
    /// B's separate referenced-only cache, just constructed for initialization.
    CoupledReferencedInitialization,
}

impl ScheduleCacheOrigin {
    /// Stable source-origin identifier, independent of schedule IDs or values.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::PipelineHourlyPrepared => "pipeline_hourly_prepared",
            Self::PipelineEnvironmentPrepared => "pipeline_environment_prepared",
            Self::CoupledProductionArgument => "coupled_production_argument",
            Self::HeatBalanceReferencedInitialization => "heat_balance_referenced_initialization",
            Self::CoupledReferencedInitialization => "coupled_referenced_initialization",
        }
    }
}

/// One actual immutable cache copied without invoking any cache producer.
#[derive(Debug)]
pub struct ScheduleCacheObservation {
    /// Explicit producer or received-argument boundary.
    pub origin: ScheduleCacheOrigin,
    /// The original cache's entries, storage variants, profile and lookup index.
    pub cache: ScheduleSeriesCache,
}

/// All real boundary occurrences on the collecting thread, with no retained cap.
#[derive(Debug)]
pub struct ScheduleProductionTrace {
    /// Explicit bounded input scope supplied by the pipeline capture boundary.
    pub scope: &'static str,
    /// Actual occurrence order; repeated initializations are never deduplicated.
    pub observations: Vec<ScheduleCacheObservation>,
}

thread_local! { static ACTIVE: RefCell<Option<ScheduleProductionTrace>> = RefCell::default(); }

struct Guard(Option<ScheduleProductionTrace>);
impl Drop for Guard {
    fn drop(&mut self) {
        ACTIVE.with_borrow_mut(|active| *active = self.0.take());
    }
}

/// Root's bounded Full capture supplies a scope; ordinary routes supply None.
/// Nesting and unwinding restore the previous collecting owner.
pub fn capture<R>(
    scope: Option<&'static str>,
    execute: impl FnOnce() -> R,
) -> (R, Option<ScheduleProductionTrace>) {
    let Some(scope) = scope else {
        return (execute(), None);
    };
    let previous = ACTIVE.with_borrow_mut(|active| {
        active.replace(ScheduleProductionTrace {
            scope,
            observations: Vec::new(),
        })
    });
    let _guard = Guard(previous);
    let result = execute();
    (result, ACTIVE.with_borrow_mut(Option::take))
}

/// Borrows an already-created owner; inactive collection does not clone it.
pub fn record_cache(origin: ScheduleCacheOrigin, cache: &ScheduleSeriesCache) {
    ACTIVE.with_borrow_mut(|active| {
        if let Some(trace) = active {
            trace.observations.push(ScheduleCacheObservation {
                origin,
                cache: cache.clone(),
            });
        }
    });
}
