//! Native ScheduleManager3534-3610 day-group cache on the retained manager.
use super::{SelectedScheduleData, SelectedScheduleManager, SourceScheduleIndex};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;

impl SelectedScheduleManager {
    pub(crate) fn source_day_group_min_max(
        &mut self,
        index: SourceScheduleIndex,
        group: usize,
    ) -> Result<(f64, f64), Error> {
        let schedule = self.schedules.get(index.0).ok_or(Error::Unavailable(
            "actual Schedule day-group pointer",
        ))?;
        if let SelectedScheduleData::Constant { .. } = &schedule.data {
            // Native Constant ignores the day group and does not set min/max.
            return Ok((schedule.current_value, schedule.current_value));
        }
        if !schedule.is_min_max_set {
            self.ensure_source_min_max(index)?;
        }
        let SelectedScheduleData::Detailed(detailed) = &self.schedules[index.0].data else {
            return Err(Error::Unavailable("actual Detailed day-group payload"));
        };
        let cached = *detailed.max_min_by_day_type_set.get(group).ok_or(
            Error::Unavailable("actual Detailed MaxMinByDayTypeSet group index"),
        )?;
        if !cached {
            let mut first_set = true;
            // Unused,Sun,Mon,Tue,Wed,Thu,Fri,Sat,Hol,SumDes,WinDes,Cus1,Cus2.
            const FILTERS: [[bool; 13]; 4] = [
                [false, false, true, true, true, true, true, false, false, false, false, false, false],
                [false, true, false, false, false, false, false, true, true, false, false, false, false],
                [false, false, false, false, false, false, false, false, false, true, false, false, false],
                [false, false, false, false, false, false, false, false, false, false, true, false, false],
            ];
            let filter = FILTERS.get(group).ok_or(Error::Unavailable(
                "actual Detailed dayTypeFilters group index",
            ))?;
            let mut week_checked = vec![false; self.week_schedules.len()];
            let mut day_checked = vec![false; self.day_schedules.len()];
            let SelectedScheduleData::Detailed(detailed) = &mut self.schedules[index.0].data else {
                return Err(Error::Unavailable("actual Detailed day-group payload"));
            };
            detailed.max_by_day_type[group] = 0.0;
            detailed.min_by_day_type[group] = 0.0;
            for day in 1..=366 {
                let SelectedScheduleData::Detailed(detailed) = &self.schedules[index.0].data else {
                    return Err(Error::Unavailable("actual Detailed day-group payload"));
                };
                let week_index = detailed.week_schedules[day].ok_or(Error::Unavailable(
                    "actual Detailed day-group weekScheds[iDay] pointer",
                ))?;
                let week = self.week_schedules.get(week_index).ok_or(Error::Unavailable(
                    "actual Detailed day-group WeekSchedule pointer",
                ))?;
                let week_number = week.number;
                if *week_checked.get(week_number).ok_or(Error::Unavailable(
                    "actual Detailed weekSchedChecked[weekSched->Num] index",
                ))? {
                    continue;
                }
                for day_type in 1..13 {
                    if !filter[day_type] {
                        continue;
                    }
                    let day_index = self.week_schedules[week_index].day_schedules[day_type].ok_or(
                        Error::Unavailable("actual Detailed day-group dayScheds[jDayType] pointer"),
                    )?;
                    let day_schedule = self.day_schedules.get(day_index).ok_or(
                        Error::Unavailable("actual Detailed day-group DaySchedule pointer"),
                    )?;
                    let day_number = day_schedule.number;
                    if *day_checked.get(day_number).ok_or(Error::Unavailable(
                        "actual Detailed daySchedChecked[daySched->Num] index",
                    ))? {
                        continue;
                    }
                    if !day_schedule.is_min_max_set {
                        self.source_day_min_max(day_index)?;
                    }
                    let day_schedule = &self.day_schedules[day_index];
                    let (minimum, maximum) = (day_schedule.min_value, day_schedule.max_value);
                    let SelectedScheduleData::Detailed(detailed) = &mut self.schedules[index.0].data else {
                        return Err(Error::Unavailable("actual Detailed day-group payload"));
                    };
                    if first_set {
                        detailed.min_by_day_type[group] = minimum;
                        detailed.max_by_day_type[group] = maximum;
                        first_set = false;
                    } else {
                        // Actual ObjexxFCL double min takes its SECOND operand
                        // on equality; max takes its FIRST. Keep both branches.
                        detailed.min_by_day_type[group] =
                            if detailed.min_by_day_type[group] < minimum {
                                detailed.min_by_day_type[group]
                            } else {
                                minimum
                            };
                        detailed.max_by_day_type[group] =
                            if detailed.max_by_day_type[group] < maximum {
                                maximum
                            } else {
                                detailed.max_by_day_type[group]
                            };
                    }
                    day_checked[day_number] = true;
                }
                week_checked[week_number] = true;
            }
            let SelectedScheduleData::Detailed(detailed) = &mut self.schedules[index.0].data else {
                return Err(Error::Unavailable("actual Detailed day-group payload"));
            };
            detailed.max_min_by_day_type_set[group] = true;
        }
        let SelectedScheduleData::Detailed(detailed) = &self.schedules[index.0].data else {
            return Err(Error::Unavailable("actual Detailed day-group payload"));
        };
        Ok((detailed.min_by_day_type[group], detailed.max_by_day_type[group]))
    }
}
