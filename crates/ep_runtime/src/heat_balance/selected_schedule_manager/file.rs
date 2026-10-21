//! Literal Schedule:File getter, reached filesystem/cache and real annual graph.
use super::external_data::{
    NativeExternalDataDiagnostic, NativeExternalDataValue as V, decode_native_csv,
    decode_native_text_json,
};
use super::minute::SourceScheduleMinuteDiagnostic::{Continue, Severe, Warning};
use super::*;
use crate::heat_balance::selected_input_processor::SourceInputStatisticsValue as Input;
use crate::heat_balance::source_initialization::SourceExternalInputFiles;
use std::{io::Write, path::Path};

fn unavailable(message: &'static str) -> SelectedScheduleError {
    SelectedScheduleError::Unavailable(message)
}

fn file_io(path: &Path, source: std::io::Error) -> SelectedScheduleError {
    SelectedScheduleError::ExternalFile {
        path: path.to_path_buf(),
        source,
    }
}

fn defined_int(value: f64) -> Result<i32, SelectedScheduleError> {
    if !value.is_finite() || value.trunc() < i32::MIN as f64 || value.trunc() > i32::MAX as f64 {
        return Err(unavailable(
            "actual Schedule:File real-to-int conversion domain",
        ));
    }
    Ok(value as i32)
}

fn custom(
    backend: &mut Option<&mut SourceErrorBackend<'_>>,
    flags: Option<&SourceDiagnosticFlags<'_>>,
    warning: bool,
    name: &str,
    message: &str,
) -> Result<(), SelectedScheduleError> {
    schedule_diagnostic(
        backend,
        flags,
        if warning { Warning } else { Severe },
        &format!("ProcessScheduleInput: Schedule:File = {name}"),
    )?;
    schedule_diagnostic(backend, flags, Continue, message)
}

fn parser_diagnostics(
    diagnostics: &[NativeExternalDataDiagnostic],
    warning: bool,
    name: &str,
    backend: &mut Option<&mut SourceErrorBackend<'_>>,
    flags: Option<&SourceDiagnosticFlags<'_>>,
) -> Result<(), SelectedScheduleError> {
    for diagnostic in diagnostics {
        let text = std::str::from_utf8(&diagnostic.message)
            .map_err(|_| unavailable("actual non-UTF8 external-file diagnostic byte output"))?;
        if diagnostic.continued {
            schedule_diagnostic(backend, flags, Continue, text)?;
        } else {
            custom(backend, flags, warning, name, text)?;
        }
    }
    Ok(())
}

fn extension(path: &Path) -> Result<String, SelectedScheduleError> {
    Ok(path
        .extension()
        .map(|s| {
            s.to_str()
                .ok_or_else(|| unavailable("actual external file extension text"))
        })
        .transpose()?
        .unwrap_or("")
        .to_ascii_uppercase())
}

impl SelectedScheduleManager {
    /// Exact six alpha/four numeric locals, including retaincase File Name.
    fn get_source_file_object_item(
        input: &mut SelectedInputProcessor,
        ordinal: usize,
        source_is_epjson: bool,
        locals: &mut SourceScheduleGetterLocals,
    ) -> Result<(), SelectedScheduleError> {
        let family = "Schedule:File";
        let names = input.source_raw_object_names(family)?;
        if names.len() != input.get_num_objects_found(family) {
            return Err(unavailable("actual Schedule:File original names/count"));
        }
        let name = names
            .get(ordinal)
            .cloned()
            .ok_or_else(|| unavailable("actual Schedule:File ordinal"))?;
        locals.0.fill(String::new());
        locals.1.fill(String::new());
        locals.2.fill(String::new());
        locals.3.fill(0.0);
        locals.4.fill(true);
        locals.5.fill(true);
        input.mark_object_as_used(family, &name);
        let fields = input.source_raw_object_fields(family, &name)?;
        let alphas = [
            ("name", "Name", ""),
            ("schedule_type_limits_name", "Schedule Type Limits Name", ""),
            ("file_name", "File Name", ""),
            ("column_separator", "Column Separator", "Comma"),
            ("interpolate_to_timestep", "Interpolate to Timestep", "No"),
            (
                "adjust_schedule_for_daylight_savings",
                "Adjust Schedule for Daylight Savings",
                "Yes",
            ),
        ];
        for (at, (field, title, default)) in alphas.iter().enumerate() {
            let (value, blank) = if at == 0 {
                (name.as_str(), name.is_empty())
            } else {
                match fields.get(*field) {
                    None => (*default, true),
                    Some(Input::String(text)) if text.is_empty() => (*default, true),
                    Some(Input::String(text)) => (text.as_str(), false),
                    _ => return Err(unavailable("actual Schedule:File schema string alpha")),
                }
            };
            *locals
                .0
                .get_mut(at)
                .ok_or_else(|| unavailable("actual Schedule:File alpha extent"))? = if at == 2 {
                value.to_owned()
            } else {
                value.to_ascii_uppercase()
            };
            locals.1[at] = if source_is_epjson { *field } else { *title }.into();
            locals.4[at] = blank;
        }
        for (at, (field, title, default)) in [
            ("column_number", "Column Number", 0.0),
            ("rows_to_skip_at_top", "Rows to Skip at Top", 0.0),
            ("number_of_hours_of_data", "Number of Hours of Data", 8760.0),
            ("minutes_per_item", "Minutes per Item", 60.0),
        ]
        .iter()
        .enumerate()
        {
            let (value, blank) = match fields.get(*field) {
                None => (*default, true),
                Some(Input::Number(text)) => (
                    ep_model::classify_source_json_number(text)
                        .map_err(|_| unavailable("actual Schedule:File IP numeric storage"))?
                        .as_f64(),
                    false,
                ),
                _ => return Err(unavailable("actual Schedule:File schema numeric field")),
            };
            *locals
                .3
                .get_mut(at)
                .ok_or_else(|| unavailable("actual Schedule:File numeric extent"))? = value;
            locals.2[at] = if source_is_epjson { *field } else { *title }.into();
            locals.5[at] = blank;
        }
        // Schema min_fields=10 supplies all four numeric positions for epJSON.
        // The only NumNumbers branch tests whether the true IDF max-field has N4.
        let number_count = if source_is_epjson {
            4
        } else {
            match fields.get("idf_max_fields") {
                Some(Input::Number(value)) => {
                    if value
                        .parse::<usize>()
                        .map_err(|_| unavailable("actual File IDF max fields"))?
                        > 8
                    {
                        4
                    } else {
                        3
                    }
                }
                _ => return Err(unavailable("actual Schedule:File IDF max-field metadata")),
            }
        };
        // Retain the getter's true numeric count separately, not as a blank seed.
        if number_count <= 3 {
            locals.3[3] = 60.0;
        }
        Ok(())
    }

    #[allow(
        clippy::too_many_arguments,
        reason = "Preserve ordered real IP, file, cache, graph and diagnostic owners."
    )]
    pub(super) fn process_source_files(
        &mut self,
        input: &mut SelectedInputProcessor,
        source_is_epjson: Option<bool>,
        steps: usize,
        locals: &mut SourceScheduleGetterLocals,
        mut external_files: Option<&mut SourceExternalInputFiles>,
        mut search: Option<&mut SourceFileSearchState>,
        audit: &mut dyn Write,
        backend: &mut Option<&mut SourceErrorBackend<'_>>,
        flags: Option<&SourceDiagnosticFlags<'_>>,
        errors_found: &mut bool,
    ) -> Result<(), SelectedScheduleError> {
        let count = input.get_num_objects_found("Schedule:File");
        if count == 0 {
            return Ok(());
        }
        let source_is_epjson =
            source_is_epjson.ok_or_else(|| unavailable("actual File input format owner"))?;
        let slots = steps
            .checked_mul(24)
            .ok_or_else(|| unavailable("actual File timestep extent"))?;
        for ordinal in 0..count {
            Self::get_source_file_object_item(input, ordinal, source_is_epjson, locals)?;
            let name = locals.0[0].clone();
            if self.schedule_map.contains_key(&name) {
                schedule_diagnostic(
                    backend,
                    flags,
                    Severe,
                    &format!("ProcessScheduleInput: Schedule:File = {name}, duplicate name."),
                )?;
                *errors_found = true;
                continue;
            }
            let schedule = self.add_source_detailed(&name);
            let SelectedScheduleData::Detailed(detail) = &mut self.schedules[schedule.0].data
            else {
                return Err(unavailable("actual File Detailed allocation"));
            };
            detail.schedule_kind = 2;
            let type_number = if locals.4[1] {
                None
            } else {
                self.schedule_type_map.get(&locals.0[1]).copied()
            };
            self.schedules[schedule.0].schedule_type_number = type_number;
            if type_number.is_none() {
                let message = if locals.4[1] {
                    format!("{} is empty.", locals.1[1])
                } else {
                    format!("{} = {}, item not found", locals.1[1], locals.0[1])
                };
                custom(backend, flags, true, &name, &message)?;
                schedule_diagnostic(backend, flags, Continue, "Schedule will not be validated.")?;
            }
            let column = defined_int(locals.3[0])?;
            let skip = defined_int(locals.3[1])?;
            if locals.3[2] == 0.0 {
                locals.3[2] = 8760.0;
            }
            let hours = locals.3[2];
            if hours != 8760.0 && hours != 8784.0 {
                let formatted = crate::heat_balance::source_round_format::truncate(hours, 0)
                    .map_err(|_| unavailable("actual File invalid hours .0T formatter"))?;
                custom(
                    backend,
                    flags,
                    false,
                    &name,
                    &format!(
                        "{} must = 8760 or 8784 (for a leap year).  Value = {formatted}, Schedule not processed.",
                        locals.2[2]
                    ),
                )?;
                *errors_found = true;
                continue;
            }
            let delimiter = if locals.4[3] || locals.0[3].eq_ignore_ascii_case("comma") {
                locals.0[3] = "comma".into();
                b','
            } else {
                match locals.0[3].as_str() {
                    "SEMICOLON" => b';',
                    "TAB" => b'\t',
                    "SPACE" => b' ',
                    _ => {
                        custom(
                            backend,
                            flags,
                            false,
                            &name,
                            &format!("{} = {}, invalid key.", locals.1[3], locals.0[3]),
                        )?;
                        schedule_diagnostic(
                            backend,
                            flags,
                            Continue,
                            "..must be Comma, Semicolon, Tab, or Space.",
                        )?;
                        *errors_found = true;
                        continue;
                    }
                }
            };
            let mut interpolation = 0;
            if !locals.4[4] {
                match locals.0[4].as_str() {
                    "YES" => interpolation = 1,
                    "NO" => interpolation = 2,
                    _ => {
                        custom(
                            backend,
                            flags,
                            false,
                            &name,
                            &format!("{} = {}, invalid key.", locals.1[4], locals.0[4]),
                        )?;
                        *errors_found = true;
                    }
                }
            }
            let SelectedScheduleData::Detailed(detail) = &mut self.schedules[schedule.0].data
            else {
                return Err(unavailable("actual File Detailed type"));
            };
            detail.use_daylight_saving = locals.0[5] != "NO";
            let minutes = defined_int(locals.3[3])?;
            if minutes == 0 {
                return Err(unavailable("actual Schedule:File undefined modulo zero"));
            }
            if 60 % minutes != 0 {
                custom(
                    backend,
                    flags,
                    false,
                    &name,
                    &format!(
                        "Requested {} field value ({minutes}) not evenly divisible into 60",
                        locals.2[3]
                    ),
                )?;
                *errors_found = true;
                continue;
            }
            let row_limit = defined_int((hours * 60.0) / f64::from(minutes))?;
            let per_hour = 60 / minutes;
            if minutes < 1 || per_hour < 1 {
                return Err(unavailable("actual File negative minute/storage indexing"));
            }
            let context = format!("Schedule:File=\"{name}\", {}: ", locals.1[2]);
            let original_path = Path::new(&locals.0[2]);
            let files = external_files
                .as_deref_mut()
                .ok_or_else(|| unavailable("actual Schedule:File CLI/file IO owner"))?;
            let search = search
                .as_deref_mut()
                .ok_or_else(|| unavailable("actual SystemVars external-file search owner"))?;
            let outcome = files
                .resolve(original_path, search, audit)
                .map_err(|e| file_io(original_path, e))?;
            let Some(path) = outcome.found else {
                schedule_diagnostic(
                    backend,
                    flags,
                    Severe,
                    &format!("{context}\"{}\" not found.", locals.0[2]),
                )?;
                schedule_diagnostic(backend, flags, Continue, "  Paths searched:")?;
                for (label, parent) in outcome.paths_searched {
                    let text = parent
                        .to_str()
                        .ok_or_else(|| unavailable("actual path search diagnostic encoding"))?;
                    schedule_diagnostic(
                        backend,
                        flags,
                        Continue,
                        &format!("    {label}: \"{text}\""),
                    )?;
                }
                // Native assigns the empty helper result only after all missing-path diagnostics succeed.
                files.temp_full_file_path.clear();
                *errors_found = true;
                continue;
            };
            let key = SourceNativePathKey::new(&path).map_err(|e| file_io(&path, e))?;
            if !self.unique_processed_external_files.contains_key(&key) {
                let ext = extension(&path)?;
                if matches!(
                    ext.as_str(),
                    "IDF" | "IMF" | "CSV" | "TSV" | "TXT" | "ESO" | "MTR"
                ) {
                    let bytes = files.read_bytes().map_err(|e| file_io(&path, e))?;
                    let parsed = decode_native_csv(&bytes, delimiter, skip);
                    self.unique_processed_external_files
                        .insert(key.clone(), parsed.value);
                    if let Some(boundary) = parsed.native_undefined_boundary {
                        return Err(unavailable(boundary.cause));
                    }
                    if !parsed.errors.is_empty() {
                        parser_diagnostics(&parsed.errors, false, &name, backend, flags)?;
                        schedule_diagnostic(
                            backend,
                            flags,
                            Continue,
                            &format!("Error Occurred in {}", path.display()),
                        )?;
                        return schedule_fatal(
                            backend,
                            flags,
                            "Program terminates due to previous condition.",
                        );
                    }
                    parser_diagnostics(&parsed.warnings, true, &name, backend, flags)?;
                } else if matches!(ext.as_str(), "JSON" | "EPJSON" | "GLHE") {
                    let bytes = files.read_bytes().map_err(|e| file_io(&path, e))?;
                    let value = decode_native_text_json(&bytes).map_err(|_| {
                        unavailable("actual FileSystem::readJSON text parse exception")
                    })?;
                    self.unique_processed_external_files
                        .insert(key.clone(), value);
                } else if matches!(ext.as_str(), "CBOR" | "MSGPACK" | "UBJSON" | "BSON") {
                    let _opened = std::fs::File::open(&path).map_err(|e| file_io(&path, e))?;
                    return Err(unavailable("actual FileSystem binary JSON decoder"));
                } else {
                    // Native tries CSV and emplaces its partial result before testing errors.
                    let parsed = files
                        .read_bytes()
                        .ok()
                        .map(|bytes| decode_native_csv(&bytes, delimiter, skip));
                    let mut csv_ok = false;
                    if let Some(parsed) = parsed {
                        self.unique_processed_external_files
                            .entry(key.clone())
                            .or_insert(parsed.value);
                        if let Some(boundary) = parsed.native_undefined_boundary {
                            return Err(unavailable(boundary.cause));
                        }
                        csv_ok = parsed.errors.is_empty();
                    }
                    if csv_ok {
                        let backend = backend.as_deref_mut().ok_or_else(|| {
                            unavailable("actual unknown extension warning backend")
                        })?;
                        // This warning is inside Native's CSV try/catch; a reached
                        // write exception falls through to the JSON extension probe.
                        csv_ok = backend.show_warning_message(&format!("Extension of file {} is unrecognized, but parsed as CSV successfully", path.display())).is_ok();
                    }
                    if !csv_ok {
                        // readJSON opens then rejects this still-unknown extension before decoding.
                        let _attempt = std::fs::File::open(&path);
                        custom(
                            backend,
                            flags,
                            false,
                            &name,
                            &format!(
                                "{} = {} has an unknown file extension and cannot be read by this program.",
                                locals.1[2], locals.0[2]
                            ),
                        )?;
                        return schedule_fatal(
                            backend,
                            flags,
                            "Program terminates due to previous condition.",
                        );
                    }
                }
            }
            let data = self
                .unique_processed_external_files
                .get_mut(&key)
                .ok_or_else(|| unavailable("actual external parsed cache iterator"))?;
            if matches!(data, V::Null) {
                *data = V::Object(BTreeMap::new());
            }
            let V::Object(object) = data else {
                return Err(unavailable(
                    "actual external JSON string-key operator[] type error",
                ));
            };
            let columns = object.entry(b"values".to_vec()).or_insert(V::Null);
            let column_count = match columns {
                V::Null => 0,
                V::Array(a) => a.len(),
                V::Object(a) => a.len(),
                _ => 1,
            };
            if column < 0 || usize::try_from(column).map_or(true, |n| n > column_count) {
                custom(
                    backend,
                    flags,
                    false,
                    &name,
                    &format!(
                        "Requested column number {column}, but found only {column_count} columns."
                    ),
                )?;
                schedule_diagnostic(
                    backend,
                    flags,
                    Continue,
                    &format!("Error Occurred in {}", path.display()),
                )?;
                return schedule_fatal(
                    backend,
                    flags,
                    "Program terminates due to previous condition.",
                );
            }
            if column == 0 {
                return Err(unavailable("actual File zero column index underflow"));
            }
            let V::Array(columns) = columns else {
                return Err(unavailable(
                    "actual external JSON numeric operator[] type error",
                ));
            };
            let column_data = &columns[column as usize - 1];
            let rows = match column_data {
                V::Null => 0,
                V::Array(a) => a.len(),
                V::Object(a) => a.len(),
                _ => 1,
            };
            if rows > i32::MAX as usize {
                return Err(unavailable(
                    "actual Schedule:File column size to int conversion domain",
                ));
            }
            let values = match column_data.real64_column() {
                Ok(values) => values,
                Err(error) => {
                    custom(
                        backend,
                        flags,
                        false,
                        &name,
                        &format!("Column number {column} has non-numeric data."),
                    )?;
                    schedule_diagnostic(
                        backend,
                        flags,
                        Continue,
                        &format!(
                            "[json.exception.type_error.302] type must be {}, but is {}",
                            error.expected, error.actual
                        ),
                    )?;
                    schedule_diagnostic(
                        backend,
                        flags,
                        Continue,
                        &format!("Error Occurred in {}", path.display()),
                    )?;
                    return schedule_fatal(
                        backend,
                        flags,
                        "Program terminates due to previous condition.",
                    );
                }
            };
            // Native numerrors is ctor0 and never assigned by the actual CSV path.
            if rows < row_limit as usize {
                let reported = rows
                    .checked_mul(60)
                    .ok_or_else(|| unavailable("actual File warning size multiplication"))?
                    / minutes as usize;
                custom(
                    backend,
                    flags,
                    true,
                    &name,
                    &format!(
                        "less than {} hourly values read from file...Number read={reported}.",
                        hours as i32
                    ),
                )?;
            }
            let mut day = 0usize;
            let mut field = 0usize;
            while day < 366 {
                day += 1;
                let day_index = self.add_source_day(format!("{name}_dy_{day}"), slots);
                self.day_schedules[day_index].schedule_type_number = type_number;
                let week = self.add_source_week(format!("{name}_wk_{day}"));
                self.week_schedules[week].day_schedules[1..].fill(Some(day_index));
                let SelectedScheduleData::Detailed(detail) = &mut self.schedules[schedule.0].data
                else {
                    return Err(unavailable("actual File annual pointer owner"));
                };
                detail.week_schedules[day] = Some(week);
                if minutes == 60 {
                    for hour in 0..24 {
                        let value = *values.get(field).ok_or_else(|| {
                            unavailable(
                                "actual Native short File selected-column unchecked indexing",
                            )
                        })?;
                        field += 1;
                        for timestep in 0..steps {
                            let at = hour * steps + timestep;
                            self.day_schedules[day_index].timestep_values[at] = value;
                            self.day_schedules[day_index].sum_timestep_values += value;
                        }
                    }
                } else {
                    let mut minute_values = [0.0; 1440];
                    for hour in 0..24 {
                        for item in 0..per_hour as usize {
                            let value = *values.get(field).ok_or_else(|| {
                                unavailable(
                                    "actual Native short File selected-column unchecked indexing",
                                )
                            })?;
                            field += 1;
                            let begin = hour * 60 + item * minutes as usize;
                            minute_values[begin..begin + minutes as usize].fill(value);
                        }
                    }
                    self.day_schedules[day_index].interpolation = interpolation;
                    minute::populate_source_day_from_minutes(
                        &mut self.day_schedules[day_index],
                        &minute_values,
                        steps,
                    )?;
                }
                if day == 59 && rows < 8784 * per_hour as usize {
                    day += 1;
                    let SelectedScheduleData::Detailed(detail) =
                        &mut self.schedules[schedule.0].data
                    else {
                        return Err(unavailable("actual File February29 pointer alias"));
                    };
                    detail.week_schedules[day] = detail.week_schedules[day - 1];
                }
            }
        }
        Ok(())
    }
}
