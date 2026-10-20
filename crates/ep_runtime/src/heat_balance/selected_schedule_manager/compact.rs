//! Native Compact1272-1521 on the sole retained IP/day/week/schedule owners.
use super::*;
use crate::heat_balance::selected_input_processor::SourceInputStatisticsValue as V;

fn unavailable(message: &'static str) -> SelectedScheduleError {
    SelectedScheduleError::Unavailable(message)
}
fn alpha(value: Option<&V>) -> Result<String, SelectedScheduleError> {
    match value {
        None | Some(V::Null) => Ok(String::new()),
        Some(V::String(text)) => Ok(text.to_ascii_uppercase()),
        Some(V::Number(text)) => {
            if !text.contains(['.', 'e', 'E']) {
                // Native IP829 i64toa. No floating-point roundtrip for integers.
                return text
                    .parse::<i64>()
                    .map(|value| value.to_string())
                    .map_err(|_| unavailable("actual Compact JSON signed integer alpha domain"));
            }
            let value = text
                .parse::<f64>()
                .ok()
                .filter(|x| x.is_finite())
                .ok_or_else(|| unavailable("actual Compact JSON finite float alpha domain"))?;
            crate::heat_balance::source_output_registration::source_json_alpha_number(value)
                .map_err(SelectedScheduleError::from)
        }
        _ => Err(unavailable("actual Compact alpha JSON type")),
    }
}
fn parsed_number(value: &str) -> Result<f64, SelectedScheduleError> {
    ep_model::process_source_number(value)
        .map_err(|_| unavailable("actual Compact Util::ProcessNumber branch"))
}
fn through_day(value: &str) -> Result<(usize, String), SelectedScheduleError> {
    let value = value
        .strip_prefix("THROUGH")
        .ok_or_else(|| unavailable("actual Compact expecting Through diagnostic"))?;
    let value = value.strip_prefix(':').unwrap_or(value).trim();
    use crate::weather::raw::{
        DateType, RawDayFailure, RawEpwHeaderError, SourceDateScratch,
        process_source_date_string_with_diagnostic, source_ordinal_day,
    };
    let mut date = SourceDateScratch::default();
    let mut errors = false;
    // Native ScheduleManager1330 calls the same real General child. Valid
    // fixed MonthDay spellings produce no diagnostic. The invalid-date effect
    // branch stays unavailable until this caller supplies its live backend.
    process_source_date_string_with_diagnostic(value, &mut date, false, &mut errors, &mut |_| {
        Err(
            RawDayFailure::Header(RawEpwHeaderError::OutsideBoundedDomain(
                "actual Compact General date diagnostic owner",
            ))
            .into(),
        )
    })
    .map_err(|_| {
        unavailable("actual Compact ProcessDateString diagnostic or signed date domain")
    })?;
    if matches!(
        date.date_type,
        DateType::NthDayInMonth | DateType::LastDayInMonth
    ) {
        return Err(unavailable(
            "actual Compact invalid Through date diagnostic",
        ));
    }
    if errors || date.date_type != DateType::MonthDay {
        return Err(unavailable(
            "actual Compact invalid Through date diagnostic",
        ));
    }
    let (month, day) = date
        .month
        .zip(date.day)
        .ok_or_else(|| unavailable("actual Compact defined General MonthDay outputs"))?;
    // Consume only the actual General MonthDay outputs above. Its plain3
    // branch deliberately skips ValidateMonthDay, so checked OrdinalDay0 may
    // also come from an invalid selected month or unselected year fields.
    let ordinal = source_ordinal_day(month, day, 1)
        .map_err(|_| unavailable("actual Compact ProcessDateString signed ordinal domain"))?;
    usize::try_from(ordinal)
        .ok()
        .filter(|ordinal| (0..=366).contains(ordinal))
        .map(|day| (day, value.to_owned())) // Actual CurrentThrough after erase/strip.
        .ok_or_else(|| unavailable("actual Compact Through ordinal domain"))
}
fn for_days(value: &str, already: &mut [bool; 13]) -> Result<[bool; 13], SelectedScheduleError> {
    // Native3075 independent substring predicates and exact duplicate writes.
    let mut these = [false; 13];
    let mut one_valid = false;
    let mut duplicate = false;
    for (key, indices) in [
        ("WEEKDAY", &[2usize, 3, 4, 5, 6][..]),
        ("MONDAY", &[2][..]),
        ("TUESDAY", &[3][..]),
        ("WEDNESDAY", &[4][..]),
        ("THURSDAY", &[5][..]),
        ("FRIDAY", &[6][..]),
        ("WEEKEND", &[1, 7][..]),
        ("SATURDAY", &[7][..]),
        ("SUNDAY", &[1][..]),
        ("CUSTOMDAY1", &[11][..]),
        ("CUSTOMDAY2", &[12][..]),
        ("ALLDAY", &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12][..]),
        ("HOLIDAY", &[8][..]),
        ("SUMMER", &[9][..]),
        ("WINTER", &[10][..]),
    ] {
        if value.contains(key) {
            for &index in indices {
                these[index] = true;
                if already[index] {
                    duplicate = true;
                } else {
                    already[index] = true;
                }
            }
            one_valid = true;
        }
    }
    if value.contains("ALLOTHERDAY") {
        for index in 0..13 {
            if !already[index] {
                these[index] = true;
                already[index] = true;
            }
        }
        one_valid = true;
    }
    if duplicate {
        return Err(unavailable(
            "actual ProcessForDayTypes duplicate diagnostic",
        ));
    }
    if !one_valid {
        return Err(unavailable(
            "actual ProcessForDayTypes no valid day diagnostic",
        ));
    }
    Ok(these)
}

impl SelectedScheduleManager {
    /// Selected value-producing part of Native getObjectItem, including its
    /// original IDF order, resets and unused erase. Diagnostic field-name reads
    /// remain explicit boundaries; no generic InputProcessor completion claim.
    pub(super) fn get_source_compact_object_item(
        input: &mut SelectedInputProcessor,
        number: usize,
        locals: &mut SourceScheduleGetterLocals,
        full: bool,
    ) -> Result<usize, SelectedScheduleError> {
        let names = input.source_raw_object_names("Schedule:Compact")?;
        if names.len() != input.get_num_objects_found("Schedule:Compact") {
            return Err(unavailable("actual Compact object-order/count association"));
        }
        let name = names
            .get(number)
            .cloned()
            .ok_or_else(|| unavailable("actual Compact getObjectItem ordinal"))?;
        locals.0.fill(String::new());
        locals.3.fill(0.0);
        if full {
            locals.1.fill(String::new());
            // Actual legacy_idd fixed alpha field titles; no extensible title
            // is read by this warning path.
            locals.1[0] = "Name".into();
            locals.1[1] = "Schedule Type Limits Name".into();
            locals.2.fill(String::new());
            locals.4.fill(true);
            locals.5.fill(true);
        }
        input.mark_object_as_used("Schedule:Compact", &name);
        let fields = input.source_raw_object_fields("Schedule:Compact", &name)?;
        let data = match fields.get("data") {
            Some(V::Array(data)) => data,
            _ => {
                return Err(unavailable(
                    "actual Compact extensible getObjectItem branch",
                ));
            }
        };
        let count = data
            .len()
            .checked_add(2)
            .ok_or_else(|| unavailable("actual Compact alpha count"))?;
        if count < 5 || count >= locals.0.len() {
            return Err(unavailable(
                "actual Compact min/max fields or sentinel association",
            ));
        }
        locals.0[0] = name.to_ascii_uppercase();
        locals.0[1] = alpha(fields.get("schedule_type_limits_name"))?;
        for (index, value) in data.iter().enumerate() {
            let V::Object(fields) = value else {
                return Err(unavailable("actual Compact extensible object"));
            };
            // The schema permits absent `field` (a real blank alpha) and
            // arbitrary other item members, but explicitly supplied null is not
            // number|string. Keep that invalid schema path unavailable.
            if matches!(fields.get("field"), Some(V::Null)) {
                return Err(unavailable("actual Compact null extensible field schema"));
            }
            locals.0[index + 2] = alpha(fields.get("field"))?;
        }
        if full {
            for index in 0..count {
                locals.4[index] = locals.0[index].is_empty();
            }
        }
        Ok(count)
    }
    fn add_source_detailed(&mut self, name: &str) -> SourceScheduleIndex {
        let number = SourceScheduleIndex(self.schedules.len());
        self.schedules.push(SelectedSchedule {
            name: name.into(),
            number,
            is_used: false,
            min_value: 0.0,
            max_value: 0.0,
            is_min_max_set: false,
            schedule_type_number: None,
            ems_actuated_on: false,
            ems_value: 0.0,
            current_value: 0.0,
            data: SelectedScheduleData::Detailed(SelectedDetailedSchedule {
                schedule_kind: 0,
                week_schedules: [None; 367],
                max_min_by_day_type_set: [false; 13],
                min_by_day_type: [0.0; 13],
                max_by_day_type: [0.0; 13],
                use_daylight_saving: true,
            }),
        });
        self.schedule_map.insert(name.to_ascii_uppercase(), number);
        number
    }
    fn add_source_week(&mut self, name: String) -> usize {
        let number = self.week_schedules.len();
        let mut days = [None; 13];
        for day in &mut days[1..] {
            *day = Some(0);
        }
        self.week_schedules.push(SelectedWeekSchedule {
            name: name.clone(),
            number,
            is_used: false,
            min_value: 0.0,
            max_value: 0.0,
            is_min_max_set: false,
            day_schedules: days,
        });
        self.week_schedule_map
            .insert(name.to_ascii_uppercase(), number);
        number
    }
    fn add_source_day(&mut self, name: String, slots: usize) -> usize {
        let number = self.day_schedules.len();
        self.day_schedules.push(SelectedDaySchedule {
            name: name.clone(),
            number,
            is_used: false,
            min_value: 0.0,
            max_value: 0.0,
            is_min_max_set: false,
            schedule_type_number: None,
            interpolation: 0,
            timestep_values: vec![0.0; slots],
            sum_timestep_values: 0.0,
        });
        self.day_schedule_map
            .insert(name.to_ascii_uppercase(), number);
        number
    }
    pub(super) fn resize_source_constant(
        &mut self,
        index: usize,
        slots: usize,
        value: f64,
    ) -> Result<(), SelectedScheduleError> {
        let row = self
            .schedules
            .get_mut(index)
            .ok_or_else(|| unavailable("actual built-in Constant pointer"))?;
        let SelectedScheduleData::Constant { timestep_values } = &mut row.data else {
            return Err(unavailable("actual built-in Constant dynamic type"));
        };
        *timestep_values = vec![value; slots];
        Ok(())
    }
    pub(super) fn process_source_compacts(
        &mut self,
        input: &mut SelectedInputProcessor,
        steps: usize,
        locals: &mut SourceScheduleGetterLocals,
        backend: &mut Option<&mut SourceErrorBackend<'_>>,
        flags: Option<&SourceDiagnosticFlags<'_>>,
        errors_found: &mut bool,
    ) -> Result<(), SelectedScheduleError> {
        if steps == 0 || steps > 60 || 60 % steps != 0 {
            return Err(unavailable("actual Compact timestep domain"));
        }
        let slots = 24 * steps;
        let mut last_for = String::new(); // Genuine ProcessScheduleInput local constructor.
        for number in 0..input.get_num_objects_found("Schedule:Compact") {
            let num_alphas = Self::get_source_compact_object_item(input, number, locals, true)?;
            if self.schedule_map.contains_key(&locals.0[0]) {
                return Err(unavailable("actual Compact duplicate-name diagnostic"));
            }
            let name = locals.0[0].clone();
            let schedule = self.add_source_detailed(&name);
            let SelectedScheduleData::Detailed(detail) = &mut self.schedules[schedule.0].data
            else {
                return Err(unavailable("actual AddDetailed type"));
            };
            detail.schedule_kind = 1;
            // Original1299-1309: keep AddDetailed/type before diagnostics;
            // a missing/unresolved optional type remains Invalid, never a
            // replacement Fraction type or a fatal input error.
            if !locals.4[1] {
                self.schedules[schedule.0].schedule_type_number =
                    self.schedule_type_map.get(&locals.0[1]).copied();
            }
            if self.schedules[schedule.0].schedule_type_number.is_none() {
                use super::minute::SourceScheduleMinuteDiagnostic::{Continue, Warning};
                schedule_diagnostic(
                    backend,
                    flags,
                    Warning,
                    &format!("ProcessScheduleInput: Schedule:Compact = {name}"),
                )?;
                let message = if locals.4[1] {
                    format!("{} is empty.", locals.1[1])
                } else {
                    format!("{} = {}, item not found.", locals.1[1], locals.0[1])
                };
                schedule_diagnostic(backend, flags, Continue, &message)?;
                schedule_diagnostic(backend, flags, Continue, "Schedule will not be validated.")?;
            }
            let mut days_in_year = [0i32; 367];
            let mut field = 2usize;
            let mut start = 1usize;
            let mut week_count = 0usize;
            let mut day_count = 0usize;
            let mut full_year = false;
            while field + 1 < num_alphas {
                let (end, current_through) = through_day(&locals.0[field])?;
                if end == 366 {
                    if full_year {
                        return Err(unavailable(
                            "actual Compact Through after full-year diagnostic",
                        ));
                    }
                    full_year = true;
                }
                week_count += 1;
                let week = self.add_source_week(format!("{name}_wk_{week_count}"));
                self.week_schedules[week].is_used = true;
                let SelectedScheduleData::Detailed(detail) = &mut self.schedules[schedule.0].data
                else {
                    return Err(unavailable("actual Compact Detailed owner"));
                };
                if start <= end {
                    // Preserve Original index order and same-row checked accesses across the retained work arrays.
                    #[allow(clippy::needless_range_loop)]
                    for day in start..=end {
                        detail.week_schedules[day] = Some(week);
                        days_in_year[day] += 1;
                    }
                }
                start = end + 1;
                let mut all_days = [false; 13];
                field += 1;
                while field + 1 < num_alphas {
                    if locals.0[field].starts_with("THROUGH") {
                        break;
                    }
                    if !locals.0[field].starts_with("FOR") {
                        return Err(unavailable("actual Compact expecting For diagnostic"));
                    }
                    day_count += 1;
                    let day = self.add_source_day(format!("{name}_dy_{day_count}"), slots);
                    self.day_schedules[day].schedule_type_number =
                        self.schedules[schedule.0].schedule_type_number;
                    self.day_schedules[day].is_used = true;
                    last_for.clone_from(&locals.0[field]); // Original1390, before the child.
                    let these_days = for_days(&locals.0[field], &mut all_days)?;
                    // Preserve Original index order and same-row checked accesses across the retained work arrays.
                    #[allow(clippy::needless_range_loop)]
                    for day_type in 1..13 {
                        if these_days[day_type] {
                            self.week_schedules[week].day_schedules[day_type] = Some(day);
                        }
                    }
                    field += 1;
                    if locals.0[field].starts_with("INTERPOLATE")
                        || !locals.0[field].starts_with("UNTIL")
                    {
                        // Original independent substring selection order.
                        self.day_schedules[day].interpolation = if locals.0[field].contains("NO") {
                            0
                        } else if locals.0[field].contains("AVERAGE") {
                            1
                        } else if locals.0[field].contains("LINEAR") {
                            2
                        } else {
                            return Err(unavailable(
                                "actual Compact invalid interpolation key diagnostic",
                            ));
                        };
                        field += 1;
                    }
                    let until_field = field;
                    let mut numbers = 0usize;
                    loop {
                        if locals.0[field].starts_with("FOR")
                            || locals.0[field].starts_with("THROUGH")
                        {
                            break;
                        }
                        if !locals.0[field].starts_with("UNTIL") {
                            return Err(unavailable("actual Compact expecting Until diagnostic"));
                        }
                        field += 1;
                        if field >= num_alphas {
                            return Err(unavailable(
                                "actual Compact value beyond getObjectItem NumAlphas",
                            ));
                        }
                        let value = parsed_number(&locals.0[field])?;
                        let target = locals
                            .3
                            .get_mut(numbers)
                            .ok_or_else(|| unavailable("actual Compact local Numbers extent"))?;
                        *target = value;
                        numbers += 1;
                        field += 1;
                        locals.0[until_field + numbers] = locals.0[field].clone();
                        if locals.0[field].is_empty() {
                            break;
                        }
                    }
                    if numbers > 0 {
                        self.populate_source_compact_day(
                            day,
                            &locals.0[until_field..until_field + numbers],
                            &locals.3[..numbers],
                            steps,
                            &name,
                            backend,
                            flags,
                            errors_found,
                        )?;
                    }
                }
                if all_days[1..].iter().any(|value| !value) {
                    use super::minute::SourceScheduleMinuteDiagnostic::{Continue, Warning};
                    // Original1480-1499: ShowWarningCustom itself emits the
                    // first Continue, followed by the three explicit rows.
                    // AddWeekSchedule already points missing slots at the real
                    // resized MissingDaySchedule0; diagnostics write no values.
                    schedule_diagnostic(
                        backend,
                        flags,
                        Warning,
                        &format!("ProcessScheduleInput: Schedule:Compact = {name}"),
                    )?;
                    schedule_diagnostic(
                        backend,
                        flags,
                        Continue,
                        &format!("has missing day types in Through={current_through}"),
                    )?;
                    schedule_diagnostic(
                        backend,
                        flags,
                        Continue,
                        &format!("Last \"For\" field={last_for}"),
                    )?;
                    const DAY_NAMES: [&str; 13] = [
                        "Unused",
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
                    ];
                    let mut message = String::from("Missing day types=,");
                    for (assigned, day_name) in all_days[1..].iter().zip(&DAY_NAMES[1..]) {
                        if !assigned {
                            let _ = message.pop();
                            message.push_str(&format!(" \"{day_name}\",-"));
                        }
                    }
                    message.truncate(message.len() - 2);
                    schedule_diagnostic(backend, flags, Continue, &message)?;
                    schedule_diagnostic(
                        backend,
                        flags,
                        Continue,
                        "Missing day types will have 0.0 as Schedule Values",
                    )?;
                }
            }
            let SelectedScheduleData::Detailed(detail) = &mut self.schedules[schedule.0].data
            else {
                return Err(unavailable("actual Compact Detailed final owner"));
            };
            if days_in_year[60] == 0 {
                days_in_year[60] = days_in_year[59];
                detail.week_schedules[60] = detail.week_schedules[59];
            }
            if days_in_year[1..].contains(&0) {
                return Err(unavailable("actual Compact missing-year-days diagnostic"));
            }
            if days_in_year[1..].iter().any(|value| *value > 1) {
                return Err(unavailable(
                    "actual Compact overlapping-year-days diagnostic",
                ));
            }
            // Positive EMS is excluded at the real preceding CheckIfAnyEMS
            // operation; no actuator or readiness state is manufactured here.
        }
        Ok(())
    }
    #[allow(clippy::too_many_arguments)]
    fn populate_source_compact_day(
        &mut self,
        day: usize,
        untils: &[String],
        values: &[f64],
        steps: usize,
        schedule_name: &str,
        backend: &mut Option<&mut SourceErrorBackend<'_>>,
        flags: Option<&SourceDiagnosticFlags<'_>>,
        schedule_errors_found: &mut bool,
    ) -> Result<(), SelectedScheduleError> {
        use super::minute::{
            SourceScheduleInterpolation, populate_source_day_from_minutes,
            process_interval_fields_with,
        };
        let row = self
            .day_schedules
            .get(day)
            .ok_or_else(|| unavailable("actual populated Compact day owner"))?;
        let mode = match row.interpolation {
            0 => SourceScheduleInterpolation::No,
            1 => SourceScheduleInterpolation::Average,
            2 => SourceScheduleInterpolation::Linear,
            _ => return Err(unavailable("actual Compact DaySchedule interpolation enum")),
        };
        let name = row.name.clone();
        let mut minute_values = [0.0; 1440];
        let mut set_minutes = [false; 1440];
        let mut errors_found = false;
        process_interval_fields_with(
            untils,
            values,
            untils.len(),
            values.len(),
            &mut minute_values,
            &mut set_minutes,
            &mut errors_found,
            &name,
            "Schedule:Compact DaySchedule Fields",
            mode,
            i32::try_from(60 / steps)
                .map_err(|_| unavailable("actual Compact minute clock conversion"))?,
            &mut |kind, message| schedule_diagnostic(backend, flags, kind, message),
        )?;
        if errors_found {
            schedule_diagnostic(
                backend,
                flags,
                super::minute::SourceScheduleMinuteDiagnostic::Continue,
                &format!("ref Schedule:Compact=\"{schedule_name}\""),
            )?;
            *schedule_errors_found = true;
        }
        // Original1467-1476 populates the same DaySchedule even when the
        // completed minute child reported input errors. Fatal is later, after
        // the full producer and ordinary schedule-type validation.

        let row = self
            .day_schedules
            .get_mut(day)
            .ok_or_else(|| unavailable("actual populated Compact day owner"))?;
        populate_source_day_from_minutes(row, &minute_values, steps)
    }
}
