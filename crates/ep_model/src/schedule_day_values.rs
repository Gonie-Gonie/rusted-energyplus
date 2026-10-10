//! Pure population of typed schedule-day segments shared by compiler and runtime.
//!
//! The minute expansion and timestep reduction retain the existing runtime arithmetic.

use crate::{ScheduleCompactSegment, ScheduleInterpolation};

/// Populates the actual zone-timestep buffer from source-ordered daily segments.
///
/// `Some(minutes_per_timestep)` must be positive and divide sixty, as supplied by
/// the existing caller timestep validation. `None` retains the unavailable empty
/// buffer. This function does not validate type limits or add missing-day owners.
#[must_use]
pub fn populate_schedule_day_values(
    interpolation: ScheduleInterpolation,
    segments: &[ScheduleCompactSegment],
    minutes_per_timestep: Option<u32>,
) -> (u32, Vec<f64>) {
    let minute_values = expand_schedule_minute_values(interpolation, segments);
    reduce_schedule_minute_values(interpolation, &minute_values, minutes_per_timestep)
}

fn reduce_schedule_minute_values(
    interpolation: ScheduleInterpolation,
    minute_values: &[f64],
    minutes_per_timestep: Option<u32>,
) -> (u32, Vec<f64>) {
    minutes_per_timestep.map_or_else(
        || (0, Vec::new()),
        |minutes_per_timestep| {
            let values = minute_values
                .chunks_exact(minutes_per_timestep as usize)
                .map(|window| match interpolation {
                    ScheduleInterpolation::Average => {
                        window.iter().sum::<f64>() / f64::from(minutes_per_timestep)
                    }
                    ScheduleInterpolation::No | ScheduleInterpolation::Linear => {
                        window.last().copied().unwrap_or(f64::NAN)
                    }
                })
                .collect();
            (minutes_per_timestep, values)
        },
    )
}

fn expand_schedule_minute_values(
    interpolation: ScheduleInterpolation,
    segments: &[ScheduleCompactSegment],
) -> Vec<f64> {
    let mut minute_values = Vec::with_capacity(1440);
    let mut previous_until_minute = 0_u32;
    let mut previous_value = None;

    for segment in segments {
        let until_minute = segment
            .until_minute_of_day
            .clamp(previous_until_minute, 1440);
        let duration_minutes = until_minute - previous_until_minute;
        if interpolation == ScheduleInterpolation::Linear {
            if let Some(start_value) = previous_value {
                let increment = (segment.value - start_value) / f64::from(duration_minutes.max(1));
                let mut current_value = start_value;
                for _minute in 1..=duration_minutes {
                    current_value += increment;
                    minute_values.push(current_value);
                }
            } else {
                minute_values.resize(until_minute as usize, segment.value);
            }
        } else {
            minute_values.resize(until_minute as usize, segment.value);
        }
        previous_until_minute = until_minute;
        previous_value = Some(segment.value);
    }

    minute_values.resize(1440, previous_value.unwrap_or(f64::NAN));
    minute_values
}
