//! Actual ScheduleManager2221-2457 EIO reports on retained Source owners.
use super::*;
use crate::heat_balance::selected_input_processor::SourceInputStatisticsValue as V;
use crate::weather::day::WeatherGlobalState;
use std::{collections::BTreeSet, io::Write};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum ReportLevel {
    Hourly,
    TimeStep,
}
impl ReportLevel {
    fn name(self) -> &'static str {
        match self {
            Self::Hourly => "Hourly",
            Self::TimeStep => "Timestep",
        }
    }
}
fn unavailable(reason: &'static str) -> SelectedScheduleError {
    SelectedScheduleError::Unavailable(reason)
}
fn real(value: f64) -> Result<String, SelectedScheduleError> {
    crate::heat_balance::source_round_format::round(value, 2)
        .map_err(|_| unavailable("actual Output:Schedules .2R format domain"))
}
fn integer(value: f64) -> Result<i32, SelectedScheduleError> {
    let truncated = value.trunc();
    if !truncated.is_finite() || truncated < f64::from(i32::MIN) || truncated > f64::from(i32::MAX)
    {
        return Err(unavailable(
            "actual Output:Schedules discrete bound int conversion",
        ));
    }
    Ok(truncated as i32)
}

impl SelectedScheduleManager {
    fn source_schedule_report_item(
        input: &mut SelectedInputProcessor,
        ordinal: usize,
        source_is_epjson: bool,
        locals: &mut SourceScheduleGetterLocals,
    ) -> Result<(), SelectedScheduleError> {
        let family = "Output:Schedules";
        let names = input.source_raw_object_names(family)?;
        if names.len() != input.get_num_objects_found(family) {
            return Err(unavailable("actual Output:Schedules raw/order count"));
        }
        let name = names
            .get(ordinal)
            .cloned()
            .ok_or_else(|| unavailable("actual Output:Schedules ordinal"))?;
        locals.0.fill(String::new());
        locals.1.fill(String::new());
        locals.2.fill(String::new());
        locals.3.fill(0.0);
        locals.4.fill(true);
        locals.5.fill(true);
        input.mark_object_as_used(family, &name);
        let fields = input.source_raw_object_fields(family, &name)?;
        let value = match fields.get("key_field") {
            None => String::new(),
            Some(V::String(text)) => text.to_ascii_uppercase(),
            _ => return Err(unavailable("actual Output:Schedules string field schema")),
        };
        *locals
            .0
            .first_mut()
            .ok_or_else(|| unavailable("actual Output:Schedules alpha extent"))? = value;
        locals.1[0] = if source_is_epjson {
            "key_field"
        } else {
            "Key Field"
        }
        .into();
        locals.4[0] = locals.0[0].is_empty();
        Ok(())
    }

    fn report_source_schedule_types(
        &self,
        eio: &mut dyn Write,
    ) -> Result<(), SelectedScheduleError> {
        writeln!(eio, "! <ScheduleTypeLimits>,Name,Limited? {{Yes/No}},Minimum,Maximum,Continuous? {{Yes/No - Discrete}}")
            .map_err(SelectedScheduleError::Report)?;
        for kind in &self.schedule_types {
            let (limited, minimum, maximum, continuous) = if kind.is_limited {
                // Native formats both .2R operands before replacing discrete
                // text with its two actual defined int conversions.
                let mut minimum = real(kind.min_value)?;
                let mut maximum = real(kind.max_value)?;
                let continuous = if kind.is_real {
                    "Yes"
                } else {
                    minimum = integer(kind.min_value)?.to_string();
                    maximum = integer(kind.max_value)?.to_string();
                    "No"
                };
                ("Yes", minimum, maximum, continuous)
            } else {
                ("No", "N/A".into(), "N/A".into(), "N/A")
            };
            writeln!(
                eio,
                "ScheduleTypeLimits,{},{},{},{},{}",
                kind.name, limited, minimum, maximum, continuous
            )
            .map_err(SelectedScheduleError::Report)?;
        }
        Ok(())
    }

    fn report_source_schedule_details(
        &self,
        level: ReportLevel,
        steps: usize,
        clock: &WeatherGlobalState,
        eio: &mut dyn Write,
    ) -> Result<(), SelectedScheduleError> {
        if usize::try_from(clock.time_steps_in_hour).ok() != Some(steps) || steps == 0 || steps > 60
        {
            return Err(unavailable("actual Output:Schedules live timestep owner"));
        }
        let slots = 24usize
            .checked_mul(steps)
            .ok_or_else(|| unavailable("actual Output:Schedules timestep extent"))?;
        let count = if level == ReportLevel::TimeStep {
            slots
        } else {
            24
        };
        let mut times = Vec::with_capacity(count);
        for hour in 0..24 {
            if level == ReportLevel::TimeStep {
                for timestep in 0..steps - 1 {
                    let minute = i32::try_from(timestep + 1)
                        .ok()
                        .and_then(|n| n.checked_mul(clock.minutes_in_time_step))
                        .ok_or_else(|| {
                            unavailable("actual Output:Schedules minute label arithmetic")
                        })?;
                    times.push(format!("{hour:02}:{minute:02}"));
                }
            }
            times.push(format!("{:02}:00", hour + 1));
        }
        if times.len() != count {
            return Err(unavailable("actual Output:Schedules time labels extent"));
        }
        let name = level.name();
        let day_table = format!("DaySchedule - {name}");
        let week_table = format!("WeekSchedule - {name}");
        let schedule_table = format!("Schedule - {name}");
        writeln!(
            eio,
            "! Schedule Details Report={name} ====================="
        )
        .map_err(SelectedScheduleError::Report)?;
        write!(
            eio,
            "! <{day_table}>,Name,ScheduleType,Interpolated {{Average/Linear/No}},Time (HH:MM) =>"
        )
        .map_err(SelectedScheduleError::Report)?;
        for time in &times {
            write!(eio, ",{time}").map_err(SelectedScheduleError::Report)?;
        }
        writeln!(eio).map_err(SelectedScheduleError::Report)?;
        write!(eio, "! <{week_table}>,Name").map_err(SelectedScheduleError::Report)?;
        for day in [
            "Sunday",
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
            "Holiday",
            "SummerDesignDay",
            "WinterDesignDay",
            "CustomDay1",
            "CustomDay2",
        ] {
            write!(eio, ",{day}").map_err(SelectedScheduleError::Report)?;
        }
        writeln!(eio).map_err(SelectedScheduleError::Report)?;
        writeln!(eio, "! <{schedule_table}>,Name,ScheduleType,Until Date 1,WeekSchedule 1,Until Date 2,WeekSchedule 2,Until Date 3,WeekSchedule 3,Until Date 4,WeekSchedule 4,Until Date 5,WeekSchedule 5,Until Date 6,WeekSchedule 6,Until Date 7,WeekSchedule 7,Until Date 8,WeekSchedule 8,Until Date 9,WeekSchedule 9,")
            .map_err(SelectedScheduleError::Report)?;
        for day in &self.day_schedules {
            let kind = match day.schedule_type_number {
                None => "",
                Some(index) => self
                    .schedule_types
                    .get(index)
                    .map(|kind| kind.name.as_str())
                    .ok_or_else(|| unavailable("actual Output:Schedules Day type pointer"))?,
            };
            let interpolation = match day.interpolation {
                0 => "No",
                1 => "Average",
                2 => "Linear",
                _ => return Err(unavailable("actual Output:Schedules interpolation enum")),
            };
            write!(
                eio,
                "{day_table},{},{},{},Values:",
                day.name, kind, interpolation
            )
            .map_err(SelectedScheduleError::Report)?;
            for hour in 0..24 {
                let first = if level == ReportLevel::Hourly {
                    steps - 1
                } else {
                    0
                };
                for timestep in first..steps {
                    let value = *day
                        .timestep_values
                        .get(hour * steps + timestep)
                        .ok_or_else(|| {
                            unavailable("actual Output:Schedules Day timestep pointer")
                        })?;
                    write!(eio, ",{}", real(value)?).map_err(SelectedScheduleError::Report)?;
                }
            }
            writeln!(eio).map_err(SelectedScheduleError::Report)?;
        }
        for week in &self.week_schedules {
            write!(eio, "{week_table},{}", week.name).map_err(SelectedScheduleError::Report)?;
            for day_type in 1..13 {
                let day = week.day_schedules[day_type]
                    .and_then(|index| self.day_schedules.get(index))
                    .ok_or_else(|| unavailable("actual Output:Schedules Week Day pointer"))?;
                write!(eio, ",{}", day.name).map_err(SelectedScheduleError::Report)?;
            }
            writeln!(eio).map_err(SelectedScheduleError::Report)?;
        }
        const MONTHS: [&str; 12] = [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ];
        for schedule in &self.schedules {
            let SelectedScheduleData::Detailed(detail) = &schedule.data else {
                continue;
            };
            let kind = match schedule.schedule_type_number {
                None => "",
                Some(index) => self
                    .schedule_types
                    .get(index)
                    .map(|kind| kind.name.as_str())
                    .ok_or_else(|| unavailable("actual Output:Schedules Detailed type pointer"))?,
            };
            write!(eio, "{schedule_table},{},{}", schedule.name, kind)
                .map_err(SelectedScheduleError::Report)?;
            let (mut month, mut date) = (0, 0);
            let mut day = 1;
            while day <= 366 {
                let week_index = detail.week_schedules[day]
                    .ok_or_else(|| unavailable("actual Output:Schedules annual Week pointer"))?;
                let week = self
                    .week_schedules
                    .get(week_index)
                    .ok_or_else(|| unavailable("actual Output:Schedules Week owner"))?;
                while detail.week_schedules[day] == Some(week_index) && day <= 366 {
                    if day == 366 {
                        crate::weather::day::source_weather_initialization::source_inverse_ordinal_day(day as i32, &mut month, &mut date, 1)?;
                        let month_name = MONTHS
                            .get(usize::try_from(month - 1).map_err(|_| {
                                unavailable("actual Output:Schedules inverse month index")
                            })?)
                            .ok_or_else(|| {
                                unavailable("actual Output:Schedules inverse month domain")
                            })?;
                        write!(eio, ",Through {month_name} {date:02},{}", week.name)
                            .map_err(SelectedScheduleError::Report)?;
                    }
                    day += 1;
                    if day > 366 {
                        break;
                    }
                }
                if day <= 366 {
                    crate::weather::day::source_weather_initialization::source_inverse_ordinal_day(
                        day as i32 - 1,
                        &mut month,
                        &mut date,
                        1,
                    )?;
                    let month_name = MONTHS
                        .get(usize::try_from(month - 1).map_err(|_| {
                            unavailable("actual Output:Schedules inverse month index")
                        })?)
                        .ok_or_else(|| {
                            unavailable("actual Output:Schedules inverse month domain")
                        })?;
                    write!(eio, ",Through {month_name} {date:02},{}", week.name)
                        .map_err(SelectedScheduleError::Report)?;
                }
            }
            writeln!(eio).map_err(SelectedScheduleError::Report)?;
        }
        Ok(())
    }

    #[allow(
        clippy::too_many_arguments,
        reason = "Borrow actual raw/getter, EIO, clock and live diagnostic owners at the Native report stage."
    )]
    pub(super) fn report_source_schedule_inputs(
        &self,
        input: &mut SelectedInputProcessor,
        source_is_epjson: Option<bool>,
        steps: usize,
        clock: Option<&WeatherGlobalState>,
        locals: &mut SourceScheduleGetterLocals,
        eio: &mut dyn Write,
        backend: &mut Option<&mut SourceErrorBackend<'_>>,
        flags: Option<&SourceDiagnosticFlags<'_>>,
    ) -> Result<(), SelectedScheduleError> {
        self.report_source_schedule_types(eio)?;
        let source_is_epjson = source_is_epjson
            .ok_or_else(|| unavailable("actual Output:Schedules input-format owner"))?;
        let mut levels = BTreeSet::new();
        for ordinal in 0..input.get_num_objects_found("Output:Schedules") {
            Self::source_schedule_report_item(input, ordinal, source_is_epjson, locals)?;
            let level = match locals.0[0].as_str() {
                "HOURLY" => ReportLevel::Hourly,
                "TIMESTEP" => ReportLevel::TimeStep,
                _ => {
                    return Err(unavailable(
                        "actual Output:Schedules invalid-key warning child",
                    ));
                }
            };
            if levels.insert(level) {
                self.report_source_schedule_details(
                    level,
                    steps,
                    clock
                        .ok_or_else(|| unavailable("actual Output:Schedules live report clock"))?,
                    eio,
                )?;
            } else {
                schedule_diagnostic(
                    backend,
                    flags,
                    minute::SourceScheduleMinuteDiagnostic::Warning,
                    &format!("ProcessScheduleInput: Output:Schedules = {}", locals.0[0]),
                )?;
                schedule_diagnostic(
                    backend,
                    flags,
                    minute::SourceScheduleMinuteDiagnostic::Continue,
                    &format!(
                        "Report level {} has already been processed. This report level will not be processed again.",
                        level.name()
                    ),
                )?;
            }
        }
        Ok(())
    }
}
