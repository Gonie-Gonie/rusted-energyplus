//! Immutable scalar handling for `Schedule:Constant`.

use super::{
    CachedScheduleSeries, ScheduleSampleStorage, ScheduleSeriesCache, ScheduleSeriesKind,
    ScheduleTrace,
};
use ep_model::{ScheduleConstant, TypedModel};

/// Simulates constant schedules for a fixed number of samples.
#[must_use]
pub fn simulate_constant_schedules(model: &TypedModel, sample_count: usize) -> Vec<ScheduleTrace> {
    ScheduleSeriesCache::from_entries(
        sample_count,
        model
            .schedules
            .iter()
            .map(|schedule| constant_cached_schedule_series(schedule, sample_count)),
    )
    .into_traces()
}

pub(super) fn constant_cached_schedule_series(
    schedule: &ScheduleConstant,
    sample_count: usize,
) -> CachedScheduleSeries {
    CachedScheduleSeries {
        schedule_id: schedule.id,
        schedule_name: schedule.name.0.clone(),
        kind: ScheduleSeriesKind::ConstantScalar {
            value: schedule.hourly_value,
        },
        samples: ScheduleSampleStorage::Scalar {
            value: schedule.hourly_value,
            len: sample_count,
        },
    }
}
