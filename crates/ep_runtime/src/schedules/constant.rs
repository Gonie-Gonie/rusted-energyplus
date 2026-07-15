//! Immutable scalar handling for `Schedule:Constant`.

use super::{ScheduleSeriesKind, ScheduleTrace, ScheduleValueSeries};
use ep_model::{ScheduleConstant, TypedModel};

/// Simulates constant schedules for a fixed number of samples.
#[must_use]
pub fn simulate_constant_schedules(model: &TypedModel, sample_count: usize) -> Vec<ScheduleTrace> {
    model
        .schedules
        .iter()
        .map(|schedule| ScheduleTrace {
            schedule_id: schedule.id,
            schedule_name: schedule.name.0.clone(),
            kind: ScheduleSeriesKind::ConstantScalar {
                value: schedule.hourly_value,
            },
            values: vec![schedule.hourly_value; sample_count],
        })
        .collect()
}

pub(super) fn constant_schedule_series(
    schedule: &ScheduleConstant,
    hours: impl IntoIterator<Item = u32>,
) -> ScheduleValueSeries {
    let values = hours
        .into_iter()
        .map(|_hour| schedule.hourly_value)
        .collect();
    ScheduleTrace {
        schedule_id: schedule.id,
        schedule_name: schedule.name.0.clone(),
        kind: ScheduleSeriesKind::ConstantScalar {
            value: schedule.hourly_value,
        },
        values,
    }
}
