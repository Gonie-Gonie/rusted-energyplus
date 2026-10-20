//! Native ScheduleManager803-1245 on the real IP and retained schedule graph.
use super::*;
use crate::heat_balance::selected_input_processor::SourceInputStatisticsValue as V;

fn unavailable(text: &'static str) -> SelectedScheduleError {
    SelectedScheduleError::Unavailable(text)
}

const WEEK_FIELDS: [&str; 12] = [
    "sunday_schedule_day_name",
    "monday_schedule_day_name",
    "tuesday_schedule_day_name",
    "wednesday_schedule_day_name",
    "thursday_schedule_day_name",
    "friday_schedule_day_name",
    "saturday_schedule_day_name",
    "holiday_schedule_day_name",
    "summerdesignday_schedule_day_name",
    "winterdesignday_schedule_day_name",
    "customday1_schedule_day_name",
    "customday2_schedule_day_name",
];
const WEEK_TITLES: [&str; 12] = [
    "Sunday Schedule:Day Name",
    "Monday Schedule:Day Name",
    "Tuesday Schedule:Day Name",
    "Wednesday Schedule:Day Name",
    "Thursday Schedule:Day Name",
    "Friday Schedule:Day Name",
    "Saturday Schedule:Day Name",
    "Holiday Schedule:Day Name",
    "SummerDesignDay Schedule:Day Name",
    "WinterDesignDay Schedule:Day Name",
    "CustomDay1 Schedule:Day Name",
    "CustomDay2 Schedule:Day Name",
];

fn alpha(value: Option<&V>) -> Result<(String, bool), SelectedScheduleError> {
    match value {
        None => Ok((String::new(), true)),
        Some(V::String(text)) => Ok((text.to_ascii_uppercase(), text.is_empty())),
        _ => Err(unavailable("actual regular schedule schema string alpha")),
    }
}
fn numeric(value: Option<&V>, default: Option<f64>) -> Result<(f64, bool), SelectedScheduleError> {
    match value {
        None => Ok((default.unwrap_or(0.0), true)),
        Some(V::Number(text)) => {
            // Native numeric fields use i64->Real64 for JSON integers and the
            // actual stored double otherwise; no alpha/Milo or scalar parsing.
            let value = if text.contains(['.', 'e', 'E']) {
                text.parse::<f64>().ok()
            } else {
                text.parse::<i64>().ok().map(|n| n as f64)
            };
            value
                .filter(|n| n.is_finite())
                .map(|n| (n, false))
                .ok_or_else(|| unavailable("actual regular schedule JSON numeric domain"))
        }
        _ => Err(unavailable("actual regular schedule schema numeric field")),
    }
}
fn metadata(
    fields: &std::collections::BTreeMap<String, V>,
    name: &str,
) -> Result<usize, SelectedScheduleError> {
    match fields.get(name) {
        Some(V::Number(text)) => text
            .parse()
            .map_err(|_| unavailable("actual regular schedule IDF field-count integer")),
        _ => Err(unavailable(
            "actual regular schedule staged IDF field-count owner",
        )),
    }
}

// Keep the actual Native positional loops and signed conversion predicates;
// independent local arrays and retained owners are intentionally not zipped.
#[allow(clippy::needless_range_loop, clippy::manual_range_contains)]
impl SelectedScheduleManager {
    /// Actual getObjectItem local arrays, defaults/blanks/names and usage erase.
    /// Fixed and extensible positions retain their separate Native max-fields.
    pub(super) fn get_source_regular_object_item(
        input: &mut SelectedInputProcessor,
        family: &str,
        ordinal: usize,
        source_is_epjson: bool,
        locals: &mut SourceScheduleGetterLocals,
    ) -> Result<(usize, usize), SelectedScheduleError> {
        let names = input.source_raw_object_names(family)?;
        if names.len() != input.get_num_objects_found(family) {
            return Err(unavailable("actual regular schedule raw/order count"));
        }
        let name = names
            .get(ordinal)
            .cloned()
            .ok_or_else(|| unavailable("actual regular schedule raw ordinal"))?;
        // Cache succeeded. The original resets every requested output before
        // erasing the actual object from unusedInputs.
        locals.0.fill(String::new());
        locals.1.fill(String::new());
        locals.2.fill(String::new());
        locals.3.fill(0.0);
        locals.4.fill(true);
        locals.5.fill(true);
        input.mark_object_as_used(family, &name);
        let fields = input.source_raw_object_fields(family, &name)?;
        let fixed: Vec<(String, String, bool)> = match family {
            "Schedule:Day:Hourly" => std::iter::once(("name".into(), "Name".into(), true))
                .chain(std::iter::once((
                    "schedule_type_limits_name".into(),
                    "Schedule Type Limits Name".into(),
                    true,
                )))
                .chain((1..=24).map(|n| (format!("hour_{n}"), format!("Hour {n}"), false)))
                .collect(),
            "Schedule:Day:Interval" | "Schedule:Day:List" => {
                let mut fields = vec![
                    ("name".into(), "Name".into(), true),
                    (
                        "schedule_type_limits_name".into(),
                        "Schedule Type Limits Name".into(),
                        true,
                    ),
                    (
                        "interpolate_to_timestep".into(),
                        "Interpolate to Timestep".into(),
                        true,
                    ),
                ];
                if family == "Schedule:Day:List" {
                    fields.push(("minutes_per_item".into(), "Minutes per Item".into(), false));
                }
                fields
            }
            "Schedule:Week:Compact" => vec![("name".into(), "Name".into(), true)],
            "Schedule:Week:Daily" => std::iter::once(("name".into(), "Name".into(), true))
                .chain(
                    WEEK_FIELDS
                        .iter()
                        .zip(WEEK_TITLES)
                        .map(|(f, t)| ((*f).into(), t.into(), true)),
                )
                .collect(),
            "Schedule:Year" => vec![
                ("name".into(), "Name".into(), true),
                (
                    "schedule_type_limits_name".into(),
                    "Schedule Type Limits Name".into(),
                    true,
                ),
            ],
            _ => return Err(unavailable("actual regular schedule literal getter family")),
        };
        let min_fields = match family {
            "Schedule:Day:Hourly" => 26,
            "Schedule:Week:Daily" => 13,
            "Schedule:Week:Compact" => 3,
            "Schedule:Day:Interval" | "Schedule:Day:List" => 5,
            _ => 7,
        };
        let max_fields = if source_is_epjson {
            fixed
                .iter()
                .enumerate()
                .fold(min_fields, |max, (at, (f, _, _))| {
                    if fields.contains_key(f) {
                        max.max(at + 1)
                    } else {
                        max
                    }
                })
        } else {
            metadata(fields, "idf_max_fields")?
        };
        let mut a = 0;
        let mut n = 0;
        let mut num_a = 0;
        let mut num_n = 0;
        for (position, (field, title, is_alpha)) in fixed.iter().enumerate() {
            if *is_alpha {
                let (value, blank) = if field == "name" {
                    (name.to_ascii_uppercase(), name.is_empty())
                } else {
                    let (value, blank) = alpha(fields.get(field))?;
                    if field == "interpolate_to_timestep" && blank {
                        ("NO".into(), true) // Actual schema default, with the real blank retained.
                    } else {
                        (value, blank)
                    }
                };
                *locals
                    .0
                    .get_mut(a)
                    .ok_or_else(|| unavailable("actual regular local alpha extent"))? = value;
                locals.1[a] = if source_is_epjson {
                    field.clone()
                } else {
                    title.clone()
                };
                locals.4[a] = blank;
                if field == "name" || position < max_fields {
                    num_a = a + 1;
                }
                a += 1;
            } else {
                let (value, blank) = numeric(fields.get(field), Some(0.0))?;
                *locals
                    .3
                    .get_mut(n)
                    .ok_or_else(|| unavailable("actual regular local numeric extent"))? = value;
                locals.2[n] = if source_is_epjson {
                    field.clone()
                } else {
                    title.clone()
                };
                locals.5[n] = blank;
                if position < max_fields {
                    num_n = n + 1;
                }
                n += 1;
            }
        }
        let (extension, extension_fields): (&str, &[(&str, &str, bool)]) = match family {
            "Schedule:Year" => (
                "schedule_weeks",
                &[
                    ("schedule_week_name", "Schedule:Week Name", true),
                    ("start_month", "Start Month", false),
                    ("start_day", "Start Day", false),
                    ("end_month", "End Month", false),
                    ("end_day", "End Day", false),
                ],
            ),
            "Schedule:Day:Interval" => (
                "data",
                &[
                    ("time", "Time", true),
                    ("value_until_time", "Value Until Time", false),
                ],
            ),
            "Schedule:Day:List" => ("extensions", &[("value", "Value", false)]),
            "Schedule:Week:Compact" => (
                "data",
                &[
                    ("daytype_list", "DayType List", true),
                    ("schedule_day_name", "Schedule:Day Name", true),
                ],
            ),
            _ => return Ok((num_a, num_n)),
        };
        // Native only visits genuine existing extensible objects. A min_fields
        // declaration does not manufacture an item or a missing alpha/value.
        if let Some(value) = fields.get(extension) {
            let V::Array(rows) = value else {
                return Err(unavailable("actual regular schedule extension array"));
            };
            let max_ext = if source_is_epjson {
                rows.len()
                    .checked_mul(extension_fields.len())
                    .ok_or_else(|| unavailable("actual regular extension extent"))?
            } else {
                metadata(fields, "idf_max_extensible_fields")?
            };
            let mut ext = 0;
            for row in rows {
                let V::Object(row) = row else {
                    return Err(unavailable("actual regular extension object"));
                };
                for &(field, title, is_alpha) in extension_fields {
                    if is_alpha {
                        let (value, blank) = alpha(row.get(field))?;
                        *locals.0.get_mut(a).ok_or_else(|| {
                            unavailable("actual regular extension alpha extent")
                        })? = value;
                        locals.1[a] = if source_is_epjson {
                            field.into()
                        } else {
                            title.into()
                        };
                        locals.4[a] = blank;
                        if ext < max_ext {
                            num_a = a + 1;
                        }
                        a += 1;
                    } else {
                        let (value, blank) = numeric(row.get(field), None)?;
                        *locals.3.get_mut(n).ok_or_else(|| {
                            unavailable("actual regular extension numeric extent")
                        })? = value;
                        locals.2[n] = if source_is_epjson {
                            field.into()
                        } else {
                            title.into()
                        };
                        locals.5[n] = blank;
                        if ext < max_ext {
                            num_n = n + 1;
                        }
                        n += 1;
                    }
                    ext += 1;
                }
            }
        }
        Ok((num_a, num_n))
    }

    fn regular_type(
        &self,
        family: &str,
        name: &str,
        locals: &SourceScheduleGetterLocals,
        backend: &mut Option<&mut SourceErrorBackend<'_>>,
        flags: Option<&SourceDiagnosticFlags<'_>>,
    ) -> Result<Option<usize>, SelectedScheduleError> {
        let found = if locals.4[1] {
            None
        } else {
            self.schedule_type_map.get(&locals.0[1]).copied()
        };
        if found.is_none() {
            use minute::SourceScheduleMinuteDiagnostic::{Continue, Warning};
            schedule_diagnostic(
                backend,
                flags,
                Warning,
                &format!("ProcessScheduleInput: {family} = {name}"),
            )?;
            let field = if locals.4[1] {
                format!("{} is empty.", locals.1[1])
            } else {
                format!("{} = {}, item not found.", locals.1[1], locals.0[1])
            };
            schedule_diagnostic(backend, flags, Continue, &field)?;
            schedule_diagnostic(backend, flags, Continue, "Schedule will not be validated.")?;
        }
        Ok(found)
    }

    #[allow(
        clippy::too_many_arguments,
        reason = "Borrow actual independent Day, getter-local and diagnostic owners without replacing Native state."
    )]
    fn check_source_regular_day(
        &self,
        index: usize,
        family: &str,
        name: &str,
        locals: &SourceScheduleGetterLocals,
        integer_preposition: &str,
        backend: &mut Option<&mut SourceErrorBackend<'_>>,
        flags: Option<&SourceDiagnosticFlags<'_>>,
    ) -> Result<(), SelectedScheduleError> {
        let kind = self.day_schedules[index].schedule_type_number;
        // Native const Day checks do not populate any min/max cache and
        // use strict bounds, independently of final Schedule EPS validation.
        if let Some(kind) = kind {
            let limits = &self.schedule_types[kind];
            let range = limits.is_limited
                && self.day_schedules[index]
                    .timestep_values
                    .iter()
                    .any(|v| *v < limits.min_value || *v > limits.max_value);
            let real = limits.is_real;
            if range {
                schedule_diagnostic(
                    backend,
                    flags,
                    minute::SourceScheduleMinuteDiagnostic::Warning,
                    &format!("ProcessScheduleInput: {family} = {name}"),
                )?;
                schedule_diagnostic(
                    backend,
                    flags,
                    minute::SourceScheduleMinuteDiagnostic::Continue,
                    &format!(
                        "Values are outside of range for {}={}",
                        locals.1[1], locals.0[1]
                    ),
                )?;
            }
            if !real {
                let mut bad = false;
                for value in &self.day_schedules[index].timestep_values {
                    let truncated = value.trunc();
                    if truncated < f64::from(i32::MIN) || truncated > f64::from(i32::MAX) {
                        return Err(unavailable(
                            "actual DaySchedule undefined discrete real-to-int conversion",
                        ));
                    }
                    if *value != f64::from(truncated as i32) {
                        bad = true;
                        break;
                    }
                }
                if bad {
                    schedule_diagnostic(
                        backend,
                        flags,
                        minute::SourceScheduleMinuteDiagnostic::Warning,
                        &format!("ProcessScheduleInput: {family} = {name}"),
                    )?;
                    schedule_diagnostic(
                        backend,
                        flags,
                        minute::SourceScheduleMinuteDiagnostic::Continue,
                        &format!(
                            "One or more values are not integer {} {}={}",
                            integer_preposition, locals.1[1], locals.0[1]
                        ),
                    )?;
                }
            }
        }
        Ok(())
    }

    #[allow(
        clippy::too_many_arguments,
        reason = "Carry the actual parent ErrorsFound alongside existing raw, local and live diagnostic owners."
    )]
    pub(super) fn process_source_regulars(
        &mut self,
        input: &mut SelectedInputProcessor,
        source_is_epjson: Option<bool>,
        steps: usize,
        locals: &mut SourceScheduleGetterLocals,
        backend: &mut Option<&mut SourceErrorBackend<'_>>,
        flags: Option<&SourceDiagnosticFlags<'_>>,
        errors_found: &mut bool,
    ) -> Result<(), SelectedScheduleError> {
        if [
            "Schedule:Day:Hourly",
            "Schedule:Day:Interval",
            "Schedule:Day:List",
            "Schedule:Week:Daily",
            "Schedule:Week:Compact",
            "Schedule:Year",
        ]
        .iter()
        .all(|f| input.get_num_objects_found(f) == 0)
        {
            return Ok(());
        }
        let source_is_epjson = source_is_epjson
            .ok_or_else(|| unavailable("actual regular schedule loader input-format owner"))?;
        if steps == 0 || steps > 60 || 60 % steps != 0 {
            return Err(unavailable("actual regular schedule timestep domain"));
        }
        let slots = 24 * steps;
        // Original family order, before any Week/Year/Compact or Constant getter.
        for ordinal in 0..input.get_num_objects_found("Schedule:Day:Hourly") {
            let family = "Schedule:Day:Hourly";
            let (_na, _nn) = Self::get_source_regular_object_item(
                input,
                family,
                ordinal,
                source_is_epjson,
                locals,
            )?;
            let name = locals.0[0].clone();
            if self.day_schedule_map.contains_key(&name) {
                return Err(unavailable(
                    "actual Hourly duplicate-name severe diagnostic",
                ));
            }
            let index = self.add_source_day(name.clone(), slots);
            // AddDay precedes the optional-type diagnostic; preserve that owner
            // if a live callback/stream fails before values are populated.
            let kind = self.regular_type(family, &name, locals, backend, flags)?;
            self.day_schedules[index].schedule_type_number = kind;
            let day = &mut self.day_schedules[index];
            day.interpolation = 0;
            for hour in 0..24 {
                for ts in 0..steps {
                    let at = hour * steps + ts;
                    day.timestep_values[at] = locals.3[hour];
                    day.sum_timestep_values += day.timestep_values[at];
                }
            }
            self.check_source_regular_day(index, family, &name, locals, "in", backend, flags)?;
        }
        for ordinal in 0..input.get_num_objects_found("Schedule:Day:Interval") {
            let family = "Schedule:Day:Interval";
            let (na, nn) = Self::get_source_regular_object_item(
                input,
                family,
                ordinal,
                source_is_epjson,
                locals,
            )?;
            let name = locals.0[0].clone();
            if self.day_schedule_map.contains_key(&name) {
                return Err(unavailable(
                    "actual Interval duplicate-name severe diagnostic",
                ));
            }
            let index = self.add_source_day(name.clone(), slots);
            self.day_schedules[index].schedule_type_number =
                self.regular_type(family, &name, locals, backend, flags)?;
            let num_fields = na
                .checked_sub(3)
                .filter(|count| *count > 0)
                .ok_or_else(|| {
                    unavailable("actual Interval insufficient-data severe diagnostic")
                })?;
            let mode = match locals.0[2].as_str() {
                "NO" => minute::SourceScheduleInterpolation::No,
                "AVERAGE" => minute::SourceScheduleInterpolation::Average,
                "LINEAR" => minute::SourceScheduleInterpolation::Linear,
                _ => {
                    return Err(unavailable(
                        "actual Interval invalid interpolation severe diagnostic",
                    ));
                }
            };
            self.day_schedules[index].interpolation = match mode {
                minute::SourceScheduleInterpolation::No => 0,
                minute::SourceScheduleInterpolation::Average => 1,
                minute::SourceScheduleInterpolation::Linear => 2,
            };
            let mut minutes = [0.0; 1440];
            let mut set_minutes = [false; 1440];
            minute::process_interval_fields_with(
                &locals.0[3..],
                &locals.3,
                num_fields,
                nn,
                &mut minutes,
                &mut set_minutes,
                errors_found,
                &name,
                family,
                mode,
                i32::try_from(60 / steps)
                    .map_err(|_| unavailable("actual Interval minute clock"))?,
                &mut |kind, message| schedule_diagnostic(backend, flags, kind, message),
            )?;
            // Native populates even when the completed minute child set the
            // real parent ErrorsFound. Its final Fatal belongs to the caller.
            minute::populate_source_day_from_minutes(
                &mut self.day_schedules[index],
                &minutes,
                steps,
            )?;
            self.check_source_regular_day(index, family, &name, locals, "in", backend, flags)?;
        }
        for ordinal in 0..input.get_num_objects_found("Schedule:Day:List") {
            let family = "Schedule:Day:List";
            let (_na, nn) = Self::get_source_regular_object_item(
                input,
                family,
                ordinal,
                source_is_epjson,
                locals,
            )?;
            let name = locals.0[0].clone();
            if self.day_schedule_map.contains_key(&name) {
                return Err(unavailable("actual List duplicate-name severe diagnostic"));
            }
            let index = self.add_source_day(name.clone(), slots);
            self.day_schedules[index].schedule_type_number =
                self.regular_type(family, &name, locals, backend, flags)?;
            self.day_schedules[index].interpolation = match locals.0[2].as_str() {
                "NO" => 0,
                "AVERAGE" => 1,
                "LINEAR" => 2,
                _ => return Err(unavailable("actual List interpolation enum")),
            };
            // Preserve the actual Native predicate order, including its count
            // test before the truncation/division. No invalid division is made.
            let raw_minutes = locals.3[0];
            if raw_minutes <= 0.0 {
                return Err(unavailable(
                    "actual List nonpositive minutes severe diagnostic",
                ));
            }
            if nn < 25 {
                return Err(unavailable(
                    "actual List insufficient count severe diagnostic",
                ));
            }
            let truncated = raw_minutes.trunc();
            if truncated < f64::from(i32::MIN) || truncated > f64::from(i32::MAX) {
                return Err(unavailable("actual List defined minutes-to-int conversion"));
            }
            let minutes_per_item = truncated as i32;
            if minutes_per_item == 0 {
                return Err(unavailable("actual List undefined zero divisor"));
            }
            let expected = 1440 / minutes_per_item;
            if i32::try_from(nn - 1).ok() != Some(expected) {
                return Err(unavailable(
                    "actual List expected item count severe diagnostic",
                ));
            }
            if 60 % minutes_per_item != 0 {
                return Err(unavailable(
                    "actual List minutes divisibility severe diagnostic",
                ));
            }
            let minutes_per_item = usize::try_from(minutes_per_item)
                .map_err(|_| unavailable("actual List positive minute extent"))?;
            let mut minutes = [0.0; 1440];
            let (mut hour, mut begin, mut end) = (0, 0, minutes_per_item - 1);
            for field in 1..nn {
                for minute in begin..=end {
                    *minutes
                        .get_mut(hour * 60 + minute)
                        .ok_or_else(|| unavailable("actual List minute storage extent"))? =
                        locals.3[field];
                }
                begin = end + 1;
                end += minutes_per_item;
                if end >= 60 {
                    end = minutes_per_item - 1;
                    begin = 0;
                    hour += 1;
                }
            }
            // Linear has no ramp producer for List; it samples the exact same
            // filled minute owner as No. Average alone accumulates every minute.
            minute::populate_source_day_from_minutes(
                &mut self.day_schedules[index],
                &minutes,
                steps,
            )?;
            self.check_source_regular_day(index, family, &name, locals, "for", backend, flags)?;
        }
        for ordinal in 0..input.get_num_objects_found("Schedule:Week:Daily") {
            let family = "Schedule:Week:Daily";
            let (na, _) = Self::get_source_regular_object_item(
                input,
                family,
                ordinal,
                source_is_epjson,
                locals,
            )?;
            let name = locals.0[0].clone();
            if self.week_schedule_map.contains_key(&name) {
                return Err(unavailable(
                    "actual WeekDaily duplicate-name severe diagnostic",
                ));
            }
            let week = self.add_source_week(name);
            if na != 13 {
                return Err(unavailable("actual WeekDaily12 required alpha references"));
            }
            for day_type in 1..13 {
                let day = self
                    .day_schedule_map
                    .get(&locals.0[day_type])
                    .copied()
                    .ok_or_else(|| {
                        unavailable("actual WeekDaily missing Day reference severe diagnostic")
                    })?;
                self.day_schedules[day].is_used = true; // Actual GetDaySchedule.
                self.week_schedules[week].day_schedules[day_type] = Some(day);
            }
        }
        for ordinal in 0..input.get_num_objects_found("Schedule:Week:Compact") {
            let family = "Schedule:Week:Compact";
            let (na, _) = Self::get_source_regular_object_item(
                input,
                family,
                ordinal,
                source_is_epjson,
                locals,
            )?;
            let name = locals.0[0].clone();
            if self.week_schedule_map.contains_key(&name) {
                return Err(unavailable(
                    "actual WeekCompact duplicate-name severe diagnostic",
                ));
            }
            let week = self.add_source_week(name);
            if na < 3 || na.is_multiple_of(2) {
                return Err(unavailable(
                    "actual WeekCompact paired reference alpha extent",
                ));
            }
            let mut all_days = [false; 13];
            for field in (1..na).step_by(2) {
                // GetDaySchedule marks the actual owner before any selector
                // side effect. Invalid selectors cannot backfill Week pointers.
                let day = self
                    .day_schedule_map
                    .get(&locals.0[field + 1])
                    .copied()
                    .ok_or_else(|| {
                        unavailable("actual WeekCompact missing Day severe diagnostic")
                    })?;
                self.day_schedules[day].is_used = true;
                let these = compact::for_days(&locals.0[field], &mut all_days)?;
                for day_type in 1..13 {
                    if these[day_type] {
                        self.week_schedules[week].day_schedules[day_type] = Some(day);
                    }
                }
            }
            if all_days[1..].iter().any(|assigned| !assigned) {
                return Err(unavailable(
                    "actual WeekCompact missing assignments severe diagnostic",
                ));
            }
        }
        for ordinal in 0..input.get_num_objects_found("Schedule:Year") {
            let family = "Schedule:Year";
            let (na, nn) = Self::get_source_regular_object_item(
                input,
                family,
                ordinal,
                source_is_epjson,
                locals,
            )?;
            let name = locals.0[0].clone();
            if self.schedule_map.contains_key(&name) {
                return Err(unavailable("actual Year duplicate-name severe diagnostic"));
            }
            let schedule = self.add_source_detailed(&name);
            let kind = self.regular_type(family, &name, locals, backend, flags)?;
            self.schedules[schedule.0].schedule_type_number = kind;
            if na < 3 || nn != 4 * (na - 2) {
                return Err(unavailable("actual Year complete5-field period arguments"));
            }
            let mut days = [0_u32; 367];
            let mut pointer = 0;
            for alpha in 2..na {
                // Missing references are outside this completing branch. Stop
                // before the Native severe child, rather than consuming dates
                // or advancing NumPointer on that continued-error path.
                let week = self
                    .week_schedule_map
                    .get(&locals.0[alpha])
                    .copied()
                    .ok_or_else(|| {
                        unavailable("actual Year missing Week reference severe diagnostic")
                    })?;
                // Actual GetWeekSchedule marks the reachable Day owners only
                // on its first use, before reading or advancing NumPointer.
                if !self.week_schedules[week].is_used {
                    self.week_schedules[week].is_used = true;
                    for day_type in 1..13 {
                        let day =
                            self.week_schedules[week].day_schedules[day_type].ok_or_else(|| {
                                unavailable("actual Year GetWeekSchedule Day pointer")
                            })?;
                        self.day_schedules[day].is_used = true;
                    }
                }
                let mut dates = [0_i32; 4];
                for (offset, target) in dates.iter_mut().enumerate() {
                    let value = locals.3[pointer + offset].trunc();
                    if value < f64::from(i32::MIN) || value > f64::from(i32::MAX) {
                        return Err(unavailable(
                            "actual Year defined numeric-to-int date conversion",
                        ));
                    }
                    *target = value as i32;
                }
                pointer += 4;
                let start = crate::weather::raw::source_ordinal_day(dates[0], dates[1], 1)
                    .map_err(|_| unavailable("actual Year start OrdinalDay arithmetic"))?;
                let end = crate::weather::raw::source_ordinal_day(dates[2], dates[3], 1)
                    .map_err(|_| unavailable("actual Year end OrdinalDay arithmetic"))?;
                if !(1..=366).contains(&start) || !(1..=366).contains(&end) {
                    return Err(unavailable("actual Year schedule array/date bounds"));
                }
                let SelectedScheduleData::Detailed(detail) = &mut self.schedules[schedule.0].data
                else {
                    return Err(unavailable("actual Year Detailed constructor"));
                };
                if start <= end {
                    for day in start..=end {
                        days[day as usize] += 1;
                        detail.week_schedules[day as usize] = Some(week);
                    }
                } else {
                    for day in start..=366 {
                        days[day as usize] += 1;
                        detail.week_schedules[day as usize] = Some(week);
                    }
                    for day in 1..=end {
                        days[day as usize] += 1;
                        detail.week_schedules[day as usize] = Some(week);
                    }
                }
            }
            let SelectedScheduleData::Detailed(detail) = &mut self.schedules[schedule.0].data
            else {
                return Err(unavailable("actual Year final Detailed owner"));
            };
            if days[60] == 0 {
                days[60] = days[59];
                detail.week_schedules[60] = detail.week_schedules[59];
            }
            for count in &days[1..] {
                if *count == 0 {
                    return Err(unavailable("actual Year missing-days severe diagnostic"));
                }
                if *count > 1 {
                    return Err(unavailable("actual Year overlap severe diagnostic"));
                }
            }
            // Positive EMS is still rejected at its real earlier input owner.
        }
        Ok(())
    }
}
