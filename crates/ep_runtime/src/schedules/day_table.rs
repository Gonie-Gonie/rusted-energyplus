//! Immutable `Schedule:Day` tables and annual week/day lookup.

use super::{
    ScheduleSeriesKind, ScheduleTrace, ScheduleValueSeries,
    detailed_schedule_environment_lookup_state, detailed_schedule_lookup_state,
    precompile_schedule_day_values, schedule_day_type_index, schedule_minutes_per_timestep,
};
use crate::time_axis::{EnvironmentTimeAxis, TimeAxis};
use ep_model::{
    DayScheduleId, ScheduleDayInterval, ScheduleDayList, ScheduleDayType, ScheduleInterpolation,
    ScheduleYear, TypedModel, WeekScheduleId,
};

/// One immutable day profile prepared at zone-timestep resolution.
#[derive(Clone, Debug, PartialEq)]
struct CompiledDaySchedule {
    day_schedule_id: DayScheduleId,
    zone_timestep_values: Vec<f64>,
}

/// Dense, fail-closed day-profile table indexed directly by `DayScheduleId`.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct CompiledDayScheduleTable {
    minutes_per_timestep: u32,
    schedules: Vec<Option<CompiledDaySchedule>>,
}

impl CompiledDayScheduleTable {
    pub(super) fn resolved_schedule_count(&self) -> usize {
        self.schedules.iter().flatten().count()
    }
}

pub(super) fn precompile_day_schedule_table(
    model: &TypedModel,
    timesteps_per_hour: u32,
) -> CompiledDayScheduleTable {
    let minutes_per_timestep = schedule_minutes_per_timestep(timesteps_per_hour);
    let table_len = model
        .day_schedules
        .len()
        .saturating_add(model.day_interval_schedules.len())
        .saturating_add(model.day_list_schedules.len());
    let mut schedules = vec![None; table_len];
    let mut ambiguous_ids = vec![false; table_len];

    for schedule in &model.day_schedules {
        let zone_timestep_values = minutes_per_timestep.map_or_else(Vec::new, |_minutes| {
            let repeats = usize::try_from(timesteps_per_hour).unwrap_or_default();
            schedule
                .hourly_values
                .iter()
                .flat_map(|value| std::iter::repeat_n(*value, repeats))
                .collect()
        });
        insert_compiled_day_schedule(
            &mut schedules,
            &mut ambiguous_ids,
            CompiledDaySchedule {
                day_schedule_id: schedule.id,
                zone_timestep_values,
            },
        );
    }

    for schedule in &model.day_interval_schedules {
        let (_minutes_per_timestep, zone_timestep_values) = precompile_schedule_day_values(
            schedule.interpolation,
            &schedule.segments,
            minutes_per_timestep,
        );
        insert_compiled_day_schedule(
            &mut schedules,
            &mut ambiguous_ids,
            CompiledDaySchedule {
                day_schedule_id: schedule.id,
                zone_timestep_values,
            },
        );
    }

    for schedule in &model.day_list_schedules {
        let zone_timestep_values =
            precompile_schedule_day_list_values(schedule, minutes_per_timestep);
        insert_compiled_day_schedule(
            &mut schedules,
            &mut ambiguous_ids,
            CompiledDaySchedule {
                day_schedule_id: schedule.id,
                zone_timestep_values,
            },
        );
    }

    CompiledDayScheduleTable {
        minutes_per_timestep: minutes_per_timestep.unwrap_or(0),
        schedules,
    }
}

fn precompile_schedule_day_list_values(
    schedule: &ScheduleDayList,
    minutes_per_timestep: Option<u32>,
) -> Vec<f64> {
    let (Some(minutes_per_timestep), Ok(minutes_per_item)) = (
        minutes_per_timestep,
        usize::try_from(schedule.minutes_per_item),
    ) else {
        return Vec::new();
    };
    let Ok(minutes_per_timestep_usize) = usize::try_from(minutes_per_timestep) else {
        return Vec::new();
    };
    if minutes_per_item == 0
        || minutes_per_timestep_usize == 0
        || schedule.values.len().checked_mul(minutes_per_item) != Some(1440)
    {
        return Vec::new();
    }

    // Schedule:Day:List is a flat source-value list. EnergyPlus repeats each
    // item over its fixed minute width; `Linear` does not create ramps here.
    let minute_values = schedule
        .values
        .iter()
        .flat_map(|value| std::iter::repeat_n(*value, minutes_per_item))
        .collect::<Vec<_>>();

    minute_values
        .chunks_exact(minutes_per_timestep_usize)
        .map(|window| match schedule.interpolation {
            ScheduleInterpolation::Average => {
                window.iter().sum::<f64>() / f64::from(minutes_per_timestep)
            }
            ScheduleInterpolation::No | ScheduleInterpolation::Linear => {
                window.last().copied().unwrap_or(f64::NAN)
            }
        })
        .collect()
}

fn insert_compiled_day_schedule(
    schedules: &mut [Option<CompiledDaySchedule>],
    ambiguous_ids: &mut [bool],
    schedule: CompiledDaySchedule,
) {
    let Ok(index) = usize::try_from(schedule.day_schedule_id.0) else {
        return;
    };
    let (Some(slot), Some(ambiguous)) = (schedules.get_mut(index), ambiguous_ids.get_mut(index))
    else {
        return;
    };
    if *ambiguous {
        return;
    }
    if slot.is_some() {
        *slot = None;
        *ambiguous = true;
    } else {
        *slot = Some(schedule);
    }
}

pub(super) fn compiled_day_schedule_value(
    table: &CompiledDayScheduleTable,
    day_schedule_id: DayScheduleId,
    minute_of_day: u32,
) -> Option<f64> {
    let schedule_index = usize::try_from(day_schedule_id.0).ok()?;
    let schedule = table
        .schedules
        .get(schedule_index)?
        .as_ref()
        .filter(|candidate| candidate.day_schedule_id == day_schedule_id)?;
    let value_index = minute_of_day
        .clamp(1, 1440)
        .checked_sub(1)?
        .checked_div(table.minutes_per_timestep)?;
    schedule
        .zone_timestep_values
        .get(value_index as usize)
        .copied()
}

fn day_interval_schedule_for_id(
    model: &TypedModel,
    day_schedule_id: DayScheduleId,
) -> Option<&ScheduleDayInterval> {
    let hourly_count = u32::try_from(model.day_schedules.len()).ok()?;
    let interval_index = usize::try_from(day_schedule_id.0.checked_sub(hourly_count)?).ok()?;
    model
        .day_interval_schedules
        .get(interval_index)
        .filter(|schedule| schedule.id == day_schedule_id)
}

fn day_list_schedule_for_id(
    model: &TypedModel,
    day_schedule_id: DayScheduleId,
) -> Option<&ScheduleDayList> {
    let hourly_count = u32::try_from(model.day_schedules.len()).ok()?;
    let interval_count = u32::try_from(model.day_interval_schedules.len()).ok()?;
    let list_offset = hourly_count.checked_add(interval_count)?;
    let list_index = usize::try_from(day_schedule_id.0.checked_sub(list_offset)?).ok()?;
    model
        .day_list_schedules
        .get(list_index)
        .filter(|schedule| schedule.id == day_schedule_id)
}

fn week_schedule_day_schedules(
    model: &TypedModel,
    week_schedule_id: WeekScheduleId,
) -> Option<&[DayScheduleId; 12]> {
    let daily_count = u32::try_from(model.week_schedules.len()).ok()?;
    if week_schedule_id.0 < daily_count {
        let daily_index = usize::try_from(week_schedule_id.0).ok()?;
        return model
            .week_schedules
            .get(daily_index)
            .filter(|schedule| schedule.id == week_schedule_id)
            .map(|schedule| &schedule.day_schedules);
    }

    let compact_index = usize::try_from(week_schedule_id.0.checked_sub(daily_count)?).ok()?;
    model
        .week_compact_schedules
        .get(compact_index)
        .filter(|schedule| schedule.id == week_schedule_id)
        .map(|schedule| &schedule.day_schedules)
}

fn year_schedule_references_compiled_day(model: &TypedModel, schedule: &ScheduleYear) -> bool {
    schedule.week_schedules.iter().any(|week_schedule_id| {
        week_schedule_day_schedules(model, *week_schedule_id).is_some_and(|day_schedules| {
            day_schedules.iter().any(|day_schedule_id| {
                day_interval_schedule_for_id(model, *day_schedule_id).is_some()
                    || day_list_schedule_for_id(model, *day_schedule_id).is_some()
            })
        })
    })
}

fn year_schedule_requires_hourly_aggregation(model: &TypedModel, schedule: &ScheduleYear) -> bool {
    schedule.week_schedules.iter().any(|week_schedule_id| {
        let Some(day_schedules) = week_schedule_day_schedules(model, *week_schedule_id) else {
            return true;
        };
        day_schedules.iter().any(|day_schedule_id| {
            if let Some(schedule) = day_interval_schedule_for_id(model, *day_schedule_id) {
                return schedule.interpolation != ScheduleInterpolation::No
                    || schedule
                        .segments
                        .iter()
                        .any(|segment| segment.until_minute_of_day % 60 != 0);
            }
            if let Some(schedule) = day_list_schedule_for_id(model, *day_schedule_id) {
                return schedule.minutes_per_item != 60
                    || schedule.interpolation == ScheduleInterpolation::Average;
            }
            let Ok(day_schedule_index) = usize::try_from(day_schedule_id.0) else {
                return true;
            };
            model
                .day_schedules
                .get(day_schedule_index)
                .is_none_or(|candidate| candidate.id != *day_schedule_id)
        })
    })
}

fn year_schedule_series_kind(
    model: &TypedModel,
    schedule: &ScheduleYear,
    day_schedule_table: &CompiledDayScheduleTable,
) -> ScheduleSeriesKind {
    if year_schedule_references_compiled_day(model, schedule) {
        ScheduleSeriesKind::YearWeekDayCompiledProfiles {
            schedule_day_count: schedule.week_schedules.len(),
            compiled_day_schedule_count: day_schedule_table.resolved_schedule_count(),
            minutes_per_timestep: day_schedule_table.minutes_per_timestep,
        }
    } else {
        ScheduleSeriesKind::YearWeekDayHourlyDirect {
            schedule_day_count: schedule.week_schedules.len(),
        }
    }
}

pub(super) fn year_schedule_series_for_time_axis(
    model: &TypedModel,
    schedule: &ScheduleYear,
    time_axis: &TimeAxis,
    day_schedule_table: &CompiledDayScheduleTable,
) -> ScheduleValueSeries {
    let requires_hourly_aggregation = year_schedule_requires_hourly_aggregation(model, schedule);
    let values = time_axis
        .points
        .iter()
        .map(|point| {
            if requires_hourly_aggregation {
                return f64::NAN;
            }
            let (schedule_day_of_year, day_type, minute_of_day) =
                detailed_schedule_lookup_state(point);
            year_schedule_compiled_value(
                model,
                schedule,
                day_schedule_table,
                schedule_day_of_year,
                day_type,
                minute_of_day,
            )
            .unwrap_or(f64::NAN)
        })
        .collect();

    ScheduleTrace {
        schedule_id: schedule.id,
        schedule_name: schedule.name.0.clone(),
        kind: year_schedule_series_kind(model, schedule, day_schedule_table),
        values,
    }
}

pub(super) fn year_schedule_series_for_environment_time_axis(
    model: &TypedModel,
    schedule: &ScheduleYear,
    time_axis: &EnvironmentTimeAxis,
    day_schedule_table: &CompiledDayScheduleTable,
) -> ScheduleValueSeries {
    let values = time_axis
        .points
        .iter()
        .map(|point| {
            let (schedule_day_of_year, day_type, minute_of_day) =
                detailed_schedule_environment_lookup_state(point);
            year_schedule_compiled_value(
                model,
                schedule,
                day_schedule_table,
                schedule_day_of_year,
                day_type,
                minute_of_day,
            )
            .unwrap_or(f64::NAN)
        })
        .collect();

    ScheduleTrace {
        schedule_id: schedule.id,
        schedule_name: schedule.name.0.clone(),
        kind: year_schedule_series_kind(model, schedule, day_schedule_table),
        values,
    }
}

fn year_schedule_compiled_value(
    model: &TypedModel,
    schedule: &ScheduleYear,
    day_schedule_table: &CompiledDayScheduleTable,
    schedule_day_of_year: u32,
    day_type: ScheduleDayType,
    minute_of_day: u32,
) -> Option<f64> {
    let schedule_day_index = usize::try_from(schedule_day_of_year.checked_sub(1)?).ok()?;
    let week_schedule_id = *schedule.week_schedules.get(schedule_day_index)?;
    let day_schedule_id = *week_schedule_day_schedules(model, week_schedule_id)?
        .get(schedule_day_type_index(day_type))?;
    compiled_day_schedule_value(day_schedule_table, day_schedule_id, minute_of_day)
}

#[cfg(test)]
pub(super) fn year_schedule_hourly_value(
    model: &TypedModel,
    schedule: &ScheduleYear,
    schedule_day_of_year: u32,
    day_type: ScheduleDayType,
    minute_of_day: u32,
) -> Option<f64> {
    let day_schedule_table = precompile_day_schedule_table(model, 1);
    year_schedule_compiled_value(
        model,
        schedule,
        &day_schedule_table,
        schedule_day_of_year,
        day_type,
        minute_of_day,
    )
}
