//! One Native-paired cohort test for the retained Compact producer and clock lookup.
//! Fixture bytes are Rust compiler inputs; production never reads this reference.
use super::{
    SelectedScheduleInput, SelectedScheduleManager, SourceConstantScheduleInput,
    SourceScheduleTypeInput,
};
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputFamily, SourceInputStatisticsInput,
    SourceInputStatisticsValue, SourceObjectDefinition, SourceObjectShape,
};
use crate::heat_balance::source_initialization::ScheduleAuditFileHost;
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState};
use ep_model::ScheduleUnitType;
use serde_json::Value;
use std::{collections::BTreeMap, error::Error, io, path::Path};

include!("fixtures/compact_native_b.rs");

type TestResult<T> = Result<T, Box<dyn Error>>;
fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}
fn array(value: &Value) -> TestResult<&Vec<Value>> {
    Ok(value.as_array().ok_or_else(|| invalid("fixture array"))?)
}
fn object(value: &Value) -> TestResult<&serde_json::Map<String, Value>> {
    Ok(value.as_object().ok_or_else(|| invalid("fixture object"))?)
}
fn string(value: &Value) -> TestResult<&str> {
    Ok(value.as_str().ok_or_else(|| invalid("fixture string"))?)
}
fn member<'a>(value: &'a Value, name: &str) -> TestResult<&'a Value> {
    Ok(object(value)?
        .get(name)
        .ok_or_else(|| invalid("fixture missing member"))?)
}
fn usize_value(value: &Value) -> TestResult<usize> {
    Ok(usize::try_from(
        value
            .as_u64()
            .ok_or_else(|| invalid("fixture unsigned integer"))?,
    )?)
}
fn raw_value(value: &Value) -> TestResult<SourceInputStatisticsValue> {
    Ok(match value {
        Value::Null => SourceInputStatisticsValue::Null,
        Value::Bool(v) => SourceInputStatisticsValue::Bool(*v),
        Value::Number(v) => SourceInputStatisticsValue::Number(v.to_string()),
        Value::String(v) => SourceInputStatisticsValue::String(v.clone()),
        Value::Array(v) => SourceInputStatisticsValue::Array(
            v.iter().map(raw_value).collect::<TestResult<Vec<_>>>()?,
        ),
        Value::Object(v) => SourceInputStatisticsValue::Object(
            v.iter()
                .map(|(k, v)| Ok((k.clone(), raw_value(v)?)))
                .collect::<TestResult<BTreeMap<_, _>>>()?,
        ),
    })
}
fn json_size(value: &Value) -> usize {
    match value {
        Value::Null => 0,
        Value::Array(rows) => rows.len(),
        Value::Object(fields) => fields.len(),
        _ => 1,
    }
}
fn alpha(fields: &Value, name: &str) -> TestResult<Option<String>> {
    Ok(match object(fields)?.get(name) {
        None | Some(Value::Null) => None,
        Some(Value::String(v)) if v.is_empty() => None,
        Some(Value::String(v)) => Some(v.to_ascii_uppercase()),
        _ => return Err(invalid("actual schedule alpha field").into()),
    })
}
fn number(fields: &Value, name: &str) -> TestResult<Option<f64>> {
    Ok(match object(fields)?.get(name) {
        None | Some(Value::Null) => None,
        Some(Value::Number(v)) => Some(v.as_f64().ok_or_else(|| invalid("schedule number"))?),
        _ => return Err(invalid("actual schedule numeric field").into()),
    })
}
fn names(fixture: &Value, family: &str) -> TestResult<Vec<String>> {
    array(member(member(fixture, "object_order")?, family)?)?
        .iter()
        .map(|v| Ok(string(v)?.to_owned()))
        .collect()
}
fn processor(fixture: &Value, raw: &Value) -> TestResult<SelectedInputProcessor> {
    let mut definitions = Vec::new();
    for row in array(member(fixture, "definitions")?)? {
        let row = array(row)?;
        if row.len() != 6 {
            return Err(invalid("query shape width").into());
        }
        definitions.push(SourceObjectDefinition {
            object_type: string(&row[0])?.to_owned(),
            alpha_fields: usize_value(&row[1])?,
            numeric_fields: usize_value(&row[2])?,
            alpha_extensions: usize_value(&row[3])?,
            numeric_extensions: usize_value(&row[4])?,
            extension: if row[5].is_null() {
                None
            } else {
                Some(string(&row[5])?.to_owned())
            },
        });
    }
    let mut families = BTreeMap::new();
    let mut statistics = SourceInputStatisticsInput::default();
    for (family, objects) in object(raw)? {
        let mut shapes = Vec::new();
        let mut retained = BTreeMap::new();
        for (name, fields) in object(objects)? {
            shapes.push(SourceObjectShape {
                object_name: name.clone(),
                field_sizes: object(fields)?
                    .iter()
                    .map(|(k, v)| (k.clone(), json_size(v)))
                    .collect(),
            });
            retained.insert(
                name.clone(),
                object(fields)?
                    .iter()
                    .map(|(k, v)| Ok((k.clone(), raw_value(v)?)))
                    .collect::<TestResult<BTreeMap<_, _>>>()?,
            );
        }
        families.insert(family.clone(), SourceInputFamily::Objects(shapes));
        statistics.objects.insert(family.clone(), retained);
        statistics
            .object_order
            .insert(family.clone(), names(fixture, family)?);
    }
    let mut input = SelectedInputProcessor::from_admitted_input(definitions, families)?;
    input.retain_source_input_statistics(statistics)?;
    Ok(input)
}
fn schedule_declarations(fixture: &Value, raw: &Value) -> TestResult<SelectedScheduleInput> {
    let mut result = SelectedScheduleInput::default();
    for name in names(fixture, "ScheduleTypeLimits")? {
        let fields = member(member(raw, "ScheduleTypeLimits")?, &name)?;
        // All three actual declarations omit unit_type, retaining Native Invalid.
        if alpha(fields, "unit_type")?.is_some() {
            return Err(invalid("cohort type unit provenance changed").into());
        }
        result.types.push(SourceScheduleTypeInput {
            object_name: name.clone(),
            name: name.to_ascii_uppercase(),
            lower: number(fields, "lower_limit_value")?,
            upper: number(fields, "upper_limit_value")?,
            numeric_type: alpha(fields, "numeric_type")?,
            unit: ScheduleUnitType::Invalid,
        });
    }
    for name in names(fixture, "Schedule:Constant")? {
        let fields = member(member(raw, "Schedule:Constant")?, &name)?;
        result.constants.push(SourceConstantScheduleInput {
            object_name: name.clone(),
            name: name.to_ascii_uppercase(),
            type_name: alpha(fields, "schedule_type_limits_name")?,
            value: number(fields, "hourly_value")?
                .ok_or_else(|| invalid("actual constant value"))?,
        });
    }
    Ok(result)
}

#[derive(Default)]
struct MemoryAudit {
    bytes: Vec<u8>,
}
impl ScheduleAuditFileHost for MemoryAudit {
    fn audit_path(&self) -> &Path {
        Path::new("test-only-compact.audit")
    }
    fn ensure_audit_open(&mut self) -> io::Result<&mut dyn io::Write> {
        Ok(&mut self.bytes)
    }
}

#[test]
fn compact_primary_and_empty_hour_match_native_reported_timestep_bits() -> TestResult<()> {
    let fixture: Value = serde_json::from_str(NATIVE_COMPACT_COHORTS_JSON)?;
    let steps = usize_value(member(&fixture, "time_steps_in_hour")?)?;
    assert_eq!(steps, 4);
    let cohorts = array(member(&fixture, "cohorts")?)?;
    assert_eq!(cohorts.len(), 2);
    for cohort in cohorts {
        let cohort_name = string(member(cohort, "name")?)?;
        let raw = member(cohort, "raw")?;
        let mut input = processor(&fixture, raw)?;
        let declarations = schedule_declarations(&fixture, raw)?;
        let mut manager = SelectedScheduleManager::default();
        manager.init_constant_state(0)?;
        manager.process_schedule_input(
            &mut input,
            &declarations,
            steps,
            &mut MemoryAudit::default(),
        )?;
        let native_declarations = array(member(cohort, "native_declarations")?)?;
        let mut native_names = Vec::new();
        for declaration in native_declarations {
            let declaration = array(declaration)?;
            if declaration.len() != 2 {
                return Err(invalid("Native dictionary pair width").into());
            }
            native_names.push(string(&declaration[1])?.to_owned());
        }
        assert_eq!(native_names.len(), 5);
        let actual_names = manager
            .schedules
            .iter()
            .skip(2)
            .map(|s| s.name.clone())
            .collect::<Vec<_>>();
        assert_eq!(
            actual_names, native_names,
            "{cohort_name}: actual declaration allocation order"
        );
        let mut indices = Vec::new();
        for (ordinal, name) in native_names.iter().enumerate() {
            let index = manager
                .get_source_schedule(name)?
                .ok_or_else(|| invalid("actual schedule lookup"))?;
            assert_eq!(index.0, ordinal + 2, "{cohort_name}: real manager index");
            indices.push(index);
        }
        let rows = array(member(cohort, "rows")?)?;
        assert_eq!(rows.len(), 96);
        for (ordinal, row) in rows.iter().enumerate() {
            let row = array(row)?;
            if row.len() != 2 {
                return Err(invalid("Native cohort row width").into());
            }
            let tuple = array(&row[0])?;
            let values = array(&row[1])?;
            assert_eq!(tuple.len(), 8);
            assert_eq!(values.len(), indices.len());
            // Consumed operands come from the actual Native ESO tuple. No year
            // exists in that tuple. Unconsumed weather/tomorrow fields remain
            // constructor state: DST0 cannot enter tomorrow's schedule branch.
            let integer = |i: usize| -> TestResult<i32> { Ok(string(&tuple[i])?.trim().parse()?) };
            let month = integer(1)?;
            let day = integer(2)?;
            let dst = integer(3)?;
            let hour = integer(4)?;
            assert_eq!(dst, 0);
            assert_eq!(string(&tuple[7])?, "Tuesday");
            let (end_minute, fraction) = string(&tuple[6])?
                .trim()
                .split_once('.')
                .ok_or_else(|| invalid("Native end-minute token"))?;
            assert!(fraction.bytes().all(|c| c == b'0'));
            let end_minute: i32 = end_minute.parse()?;
            let minutes_per_step = 60 / i32::try_from(steps)?;
            assert_eq!(end_minute % minutes_per_step, 0);
            let clock = WeatherGlobalState {
                hour_of_day: hour,
                time_step: end_minute / minutes_per_step,
                time_steps_in_hour: i32::try_from(steps)?,
                ..WeatherGlobalState::default()
            };
            let environment = WeatherEnvironmentState {
                day_of_year_schedule: crate::weather::raw::source_ordinal_day(month, day, 1)?,
                day_of_week: 3,
                holiday_index: 0,
                dst_indicator: dst,
                ..WeatherEnvironmentState::default()
            };
            manager.update_source_schedule_values(&clock, &environment)?;
            for ((index, expected), name) in indices.iter().zip(values).zip(&native_names) {
                let expected: f64 = string(expected)?.parse()?;
                let actual = manager.source_current_schedule_value(*index)?;
                assert_eq!(
                    actual.to_bits(),
                    expected.to_bits(),
                    "{cohort_name}: {name}, Native tuple {ordinal}: {tuple:?}"
                );
            }
        }
    }
    verify_authored_compact_modes_and_diagnostic_prefix(&fixture)?;
    verify_authored_regular_schedule_graph(&fixture)?;
    verify_authored_interval_list_week_graph(&fixture)?;
    verify_authored_schedule_input_reports(&fixture)?;
    Ok(())
}

// Additional actual raw/IP producers on the same existing owner test. There
// are no Native values, precomputed day profiles or extra engine cohorts here.
fn verify_authored_regular_schedule_graph(fixture: &Value) -> TestResult<()> {
    use crate::heat_balance::error_backend::{SourceDiagnosticFlags, SourceErrorBackend};
    let week_fields = [
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
    let row = |week: &str, sm: i32, sd: i32, em: i32, ed: i32| serde_json::json!({"schedule_week_name":week,"start_month":sm,"start_day":sd,"end_month":em,"end_day":ed});
    for (periods, split) in [
        (vec![row("W", 1, 1, 12, 31)], false),
        (vec![row("W", 7, 1, 6, 30)], false), // Genuine wrap, not Compact empty range.
        (vec![row("W", 1, 1, 2, 28), row("V", 3, 1, 12, 31)], true),
    ] {
        let mut raw = fixture["cohorts"][0]["raw"].clone();
        raw["Schedule:Compact"]
            .as_object_mut()
            .ok_or("actual Compact map")?
            .remove("Internal Gain Schedule");
        let mut hourly = serde_json::Map::new();
        hourly.insert(
            "schedule_type_limits_name".into(),
            serde_json::json!("Fraction"),
        );
        for hour in 2..=24 {
            hourly.insert(
                format!("hour_{hour}"),
                serde_json::json!(if (7..=18).contains(&hour) { 1 } else { 0 }),
            );
        }
        raw["Schedule:Day:Hourly"] = serde_json::json!({"H":Value::Object(hourly),"D":{"schedule_type_limits_name":"Fraction","hour_1":0.25,"hour_2":0.25,"hour_3":0.25,"hour_4":0.25,"hour_5":0.25,"hour_6":0.25,"hour_7":0.25,"hour_8":0.25,"hour_9":0.25,"hour_10":0.25,"hour_11":0.25,"hour_12":0.25,"hour_13":0.25,"hour_14":0.25,"hour_15":0.25,"hour_16":0.25,"hour_17":0.25,"hour_18":0.25,"hour_19":0.25,"hour_20":0.25,"hour_21":0.25,"hour_22":0.25,"hour_23":0.25,"hour_24":0.25}});
        let make_week = |day: &str| {
            Value::Object(
                week_fields
                    .iter()
                    .map(|f| ((*f).into(), serde_json::json!(day)))
                    .collect(),
            )
        };
        raw["Schedule:Week:Daily"] = serde_json::json!({"W":make_week("H"),"V":make_week("D")});
        raw["Schedule:Year"] = serde_json::json!({"Internal Gain Schedule":{"schedule_type_limits_name":"Fraction","schedule_weeks":periods}});
        let mut f = fixture.clone();
        for (family, order, definition) in [
            (
                "Schedule:Compact",
                vec!["CON-01 System Availability"],
                serde_json::json!(["Schedule:Compact", 2, 0, 1, 0, "data"]),
            ),
            (
                "Schedule:Day:Hourly",
                vec!["H", "D"],
                serde_json::json!(["Schedule:Day:Hourly", 2, 24, 0, 0, null]),
            ),
            (
                "Schedule:Week:Daily",
                vec!["W", "V"],
                serde_json::json!(["Schedule:Week:Daily", 13, 0, 0, 0, null]),
            ),
            (
                "Schedule:Year",
                vec!["Internal Gain Schedule"],
                serde_json::json!(["Schedule:Year", 2, 0, 1, 4, "schedule_weeks"]),
            ),
        ] {
            f["object_order"][family] = serde_json::json!(order);
            let definitions = f["definitions"]
                .as_array_mut()
                .ok_or("actual query definitions")?;
            if !definitions.iter().any(|r| r[0] == family) {
                definitions.push(definition);
            }
        }
        let mut input = processor(&f, &raw)?;
        let mut locals: super::SourceScheduleGetterLocals = (
            vec![String::new(); 16],
            vec![String::new(); 16],
            vec![String::new(); 24],
            vec![0.0; 24],
            vec![true; 16],
            vec![true; 24],
        );
        assert_eq!(
            SelectedScheduleManager::get_source_regular_object_item(
                &mut input,
                "Schedule:Day:Hourly",
                0,
                true,
                &mut locals
            )?,
            (2, 24)
        );
        assert_eq!(locals.3[0].to_bits(), 0.0_f64.to_bits());
        assert!(locals.5[0]);
        assert!(!locals.5[1]);
        assert_eq!(locals.1[1], "schedule_type_limits_name");
        assert_eq!(locals.2[0], "hour_1");
        let mut declarations = schedule_declarations(&f, &raw)?;
        declarations.source_is_epjson = Some(true);
        let mut manager = SelectedScheduleManager::default();
        manager.init_constant_state(0)?;
        manager.process_schedule_input(
            &mut input,
            &declarations,
            4,
            &mut MemoryAudit::default(),
        )?;
        let h = *manager
            .day_schedule_map
            .get("H")
            .ok_or("actual Hourly H owner")?;
        let d = *manager
            .day_schedule_map
            .get("D")
            .ok_or("actual Hourly D owner")?;
        assert_eq!((h, d), (1, 2)); // Both precede remaining Compact-generated Day.
        assert_eq!(manager.day_schedules[h].sum_timestep_values, 48.0);
        assert!(
            manager.day_schedules[d]
                .timestep_values
                .iter()
                .all(|v| *v == 0.25)
        );
        let w = *manager
            .week_schedule_map
            .get("W")
            .ok_or("actual Week W owner")?;
        let v = *manager
            .week_schedule_map
            .get("V")
            .ok_or("actual Week V owner")?;
        assert_eq!((w, v), (0, 1));
        assert!(
            manager.week_schedules[w].day_schedules[1..]
                .iter()
                .all(|p| *p == Some(h))
        );
        assert!(
            manager.week_schedules[v].day_schedules[1..]
                .iter()
                .all(|p| *p == Some(d))
        );
        let index = manager
            .get_source_schedule("INTERNAL GAIN SCHEDULE")?
            .ok_or("actual Year schedule")?;
        let super::SelectedScheduleData::Detailed(detail) = &manager.schedules[index.0].data else {
            return Err("actual Year Detailed owner".into());
        };
        assert_eq!(index.0, 2);
        assert_eq!(detail.schedule_kind, 0);
        assert!(detail.use_daylight_saving);
        assert!(detail.week_schedules[1..=60].iter().all(|p| *p == Some(w)));
        assert!(
            detail.week_schedules[61..]
                .iter()
                .all(|p| *p == Some(if split { v } else { w }))
        );
        for ordinal in [1, 59, 60, 61, 366] {
            for day_type in 1..13 {
                for step in 0..96 {
                    let clock = WeatherGlobalState {
                        hour_of_day: step / 4 + 1,
                        time_step: step % 4 + 1,
                        time_steps_in_hour: 4,
                        ..Default::default()
                    };
                    let environment = WeatherEnvironmentState {
                        day_of_year_schedule: ordinal,
                        day_of_week: if day_type <= 7 { day_type } else { 3 },
                        holiday_index: if day_type > 7 { day_type } else { 0 },
                        ..Default::default()
                    };
                    manager.update_source_schedule_values(&clock, &environment)?;
                    let expected = if split && ordinal >= 61 {
                        0.25_f64
                    } else if (7..=18).contains(&clock.hour_of_day) {
                        1.0
                    } else {
                        0.0
                    };
                    assert_eq!(
                        manager.source_current_schedule_value(index)?.to_bits(),
                        expected.to_bits()
                    );
                }
            }
        }
        // The same raw constructor chain supplies optional-type live warnings.
        // Stop on a backend-free warning without manufacturing any callback.
        for type_name in [Some("absent type"), None] {
            let mut unknown = raw.clone();
            if let Some(type_name) = type_name {
                unknown["Schedule:Day:Hourly"]["H"]["schedule_type_limits_name"] =
                    serde_json::json!(type_name);
            } else {
                unknown["Schedule:Day:Hourly"]["H"]
                    .as_object_mut()
                    .ok_or("actual Hourly fields")?
                    .remove("schedule_type_limits_name");
            }
            let mut ip = processor(&f, &unknown)?;
            let mut retained = SelectedScheduleManager::default();
            retained.init_constant_state(0)?;
            assert!(
                retained
                    .process_schedule_input(&mut ip, &declarations, 4, &mut MemoryAudit::default())
                    .is_err()
            );
            assert_eq!(retained.day_schedules.len(), 2);
            assert!(retained.week_schedules.is_empty());
            assert!(
                retained.day_schedules[1]
                    .timestep_values
                    .iter()
                    .all(|v| v.to_bits() == 0.0_f64.to_bits())
            );
            assert!(!retained.day_schedules[1].is_min_max_set);
            let mut ip = processor(&f, &unknown)?;
            let mut retained = SelectedScheduleManager::default();
            retained.init_constant_state(0)?;
            let (mut state, mut errors, mut console, mut audit) = (
                interpolation_error_state(),
                Vec::new(),
                Vec::new(),
                MemoryAudit::default(),
            );
            let (kickoff, developer, warmup, sizing) = (false, false, true, false);
            let flags = SourceDiagnosticFlags {
                kickoff: &kickoff,
                developer: &developer,
                warmup: &warmup,
                doing_sizing: &sizing,
            };
            {
                let mut backend = SourceErrorBackend {
                    state: &mut state,
                    error_stream: Some(&mut errors),
                    console: &mut console,
                    out_unit1: None,
                    out_unit2: None,
                    f_message: None,
                    message_callback: None,
                    error_callback: None,
                    sqlite: None,
                };
                retained.process_schedule_input_with_live_backend(
                    &mut ip,
                    &declarations,
                    4,
                    &mut audit,
                    &mut backend,
                    &flags,
                )?;
            }
            assert_eq!(
                (
                    state.tracking.total_warning_errors,
                    state.tracking.total_warning_errors_during_warmup
                ),
                (1, 1)
            );
            let text = std::str::from_utf8(&errors)?;
            assert!(text.contains("ProcessScheduleInput: Schedule:Day:Hourly = H"));
            assert!(text.contains(if type_name.is_some() {
                "schedule_type_limits_name = ABSENT TYPE, item not found."
            } else {
                "schedule_type_limits_name is empty."
            }));
            assert!(text.contains("Schedule will not be validated."));
        }
        // Real const Day checks: strict range first, then integer cast warning;
        // neither changes ErrorsFound or populates a min/max cache itself.
        let mut warned = raw.clone();
        warned["Schedule:Day:Hourly"]["H"]["hour_7"] = serde_json::json!(1.5);
        warned["ScheduleTypeLimits"]["Fraction"]["numeric_type"] = serde_json::json!("Discrete");
        warned["Schedule:Day:Hourly"]["D"]["schedule_type_limits_name"] =
            serde_json::json!("Temperature");
        warned["Schedule:Year"]["Internal Gain Schedule"]["schedule_type_limits_name"] =
            serde_json::json!("Control Type");
        let mut ip = processor(&f, &warned)?;
        let declarations = {
            let mut x = schedule_declarations(&f, &warned)?;
            x.source_is_epjson = Some(true);
            x
        };
        let mut prefix = SelectedScheduleManager::default();
        prefix.init_constant_state(0)?;
        assert!(
            prefix
                .process_schedule_input(&mut ip, &declarations, 4, &mut MemoryAudit::default())
                .is_err()
        );
        assert_eq!(prefix.day_schedules[1].sum_timestep_values, 50.0);
        assert!(!prefix.day_schedules[1].is_min_max_set);
        assert!(prefix.week_schedules.is_empty());
        let mut ip = processor(&f, &warned)?;
        let mut manager = SelectedScheduleManager::default();
        manager.init_constant_state(0)?;
        let (mut state, mut errors, mut console, mut audit) = (
            interpolation_error_state(),
            Vec::new(),
            Vec::new(),
            MemoryAudit::default(),
        );
        let (kickoff, developer, warmup, sizing) = (false, false, true, false);
        let flags = SourceDiagnosticFlags {
            kickoff: &kickoff,
            developer: &developer,
            warmup: &warmup,
            doing_sizing: &sizing,
        };
        {
            let mut backend = SourceErrorBackend {
                state: &mut state,
                error_stream: Some(&mut errors),
                console: &mut console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            };
            manager.process_schedule_input_with_live_backend(
                &mut ip,
                &declarations,
                4,
                &mut audit,
                &mut backend,
                &flags,
            )?;
        }
        assert_eq!(
            (
                state.tracking.total_warning_errors,
                state.tracking.total_warning_errors_during_warmup
            ),
            (2, 2)
        );
        assert_eq!(state.tracking.total_severe_errors, 0);
        let text = std::str::from_utf8(&errors)?;
        let range = text
            .find("Values are outside of range for schedule_type_limits_name=FRACTION")
            .ok_or("actual Day strict range warning")?;
        let integer = text
            .find("One or more values are not integer in schedule_type_limits_name=FRACTION")
            .ok_or("actual Day discrete warning")?;
        assert!(range < integer);
        assert_eq!(manager.day_schedules[1].sum_timestep_values, 50.0);
    }
    Ok(())
}

// Authored raw profile tables reuse the existing production-IP helper. They
// contain no Native result seed, gated cohort call or synthetic runtime clock.
fn authored_raw(fixture: &Value, mode: &str, overlap: bool) -> Value {
    let mut raw = fixture["cohorts"][0]["raw"].clone();
    raw["ScheduleTypeLimits"]["Fraction"]["upper_limit_value"] = serde_json::json!(200);
    let fields = if overlap {
        vec![
            serde_json::json!("Through: 12/31"),
            serde_json::json!("For: AllDays"),
            serde_json::json!("Interpolate: No"),
            serde_json::json!("Until: 24:00"),
            serde_json::json!(10),
            serde_json::json!("Until: 01:00"),
            serde_json::json!(11),
        ]
    } else {
        vec![
            serde_json::json!("Through: 12/31"),
            serde_json::json!("For: AllDays"),
            serde_json::json!(format!("Interpolate: {mode}")),
            serde_json::json!("Until: 00:20"),
            serde_json::json!(10),
            serde_json::json!("Until: 01:15"),
            serde_json::json!(175),
            serde_json::json!("Until: 24:00"),
            serde_json::json!(175),
        ]
    };
    raw["Schedule:Compact"]["Internal Gain Schedule"]["data"] = Value::Array(
        fields
            .into_iter()
            .map(|v| serde_json::json!({"field":v}))
            .collect(),
    );
    raw
}

use crate::heat_balance::error_backend::{SourceErrorState, SourceErrorTracking};
fn interpolation_error_state() -> SourceErrorState {
    SourceErrorState {
        output_error_header: true,
        version_string: "authored caller".into(),
        idd_version_string: "authored input".into(),
        doing_input_processing: false,
        print_console_output: true,
        kickoff_simulation: false,
        developer_flag: false,
        warmup_flag: false,
        doing_sizing: false,
        tracking: SourceErrorTracking {
            ask_for_plant_check_on_abort: false,
            exit_during_simulations: false,
            ask_for_connections_report: false,
            ask_for_surfaces_report: false,
            total_multiplied_windows: 0,
            total_coincident_vertices: 0,
            total_degenerate_surfaces: 0,
            total_receiving_non_convex_surfaces: 0,
            total_casting_non_convex_surfaces: 0,
            match_counts: [0; 20],
            abort_processing: false,
            total_severe_errors: 0,
            total_severe_errors_during_warmup: 0,
            total_severe_errors_during_sizing: 0,
            last_severe_error: String::new(),
            total_warning_errors: 0,
            total_warning_errors_during_warmup: 0,
            total_warning_errors_during_sizing: 0,
        },
    }
}

fn verify_authored_compact_modes_and_diagnostic_prefix(fixture: &Value) -> TestResult<()> {
    use crate::heat_balance::error_backend::{SourceDiagnosticFlags, SourceErrorBackend};
    let (kickoff, developer, warmup, sizing) = (false, false, true, false);
    let flags = SourceDiagnosticFlags {
        kickoff: &kickoff,
        developer: &developer,
        warmup: &warmup,
        doing_sizing: &sizing,
    };
    // This existing owner cohort also exercises the common numeric child:
    // preserve Native error-zero vs non-finite-success before any real cast.
    use ep_model::{
        SourceNumberOutcome, process_source_number, process_source_number_outcome,
        source_finite_number_prefix,
    };
    for (text, expected) in [
        ("1e-9999", 0.0_f64),
        ("-1e-9999", -0.0),
        ("1D-9999", 0.0),
        ("-1D-9999", -0.0),
        ("0e9999", 0.0),
        ("-0D9999", -0.0),
        ("15+E0", 15.0),
    ] {
        let SourceNumberOutcome::Finite(actual) = process_source_number_outcome(text) else {
            return Err("actual finite ProcessNumber outcome".into());
        };
        assert_eq!(actual.to_bits(), expected.to_bits(), "{text}");
        assert_eq!(
            process_source_number(text).map(f64::to_bits),
            Ok(expected.to_bits()),
            "{text}"
        );
    }
    for text in [
        "nan",
        "-nan",
        "INF",
        "-infinity",
        "nan(payload)",
        "nan()",
        "nan(",
        "nan(bad!)E",
        "nan(payload)X",
        "nanE+1",
        "1e9999",
        "-1e9999",
        "1e9999X",
        "1e9999E+1",
        "+1",
        "junk",
    ] {
        assert_eq!(
            process_source_number_outcome(text),
            SourceNumberOutcome::ErrorZero,
            "{text}"
        );
        assert!(process_source_number(text).is_err(), "{text}");
    }
    for text in [
        "nanE",
        "nan+E",
        "nan-E",
        "nanE12",
        "nan(payload)E",
        "nan()D",
        "infE",
        "infinityE",
        "-infE",
        "nanD",
        "infD",
        "1e9999E",
        "1e9999+E",
        "1D9999E",
    ] {
        assert_eq!(
            process_source_number_outcome(text),
            SourceNumberOutcome::NonFinite,
            "{text}"
        );
        assert!(process_source_number(text).is_err(), "{text}");
    }
    for text in ["1e-9999", "-1e-9999", "1e9999"] {
        assert_eq!(
            source_finite_number_prefix(text),
            Err("decimal magnitude outside the finite list-read domain"),
            "{text}"
        );
    }
    assert_eq!(
        source_finite_number_prefix("nan(payload)E"),
        Err("non-finite mandatory number")
    );
    assert_eq!(source_finite_number_prefix("1D-9999"), Ok(Some((1.0, 1))));
    // Real General must stop unsupported non-finite->int before writing any
    // MonthDay or fabricated error flag/zero. Its actual diagnostic is unreached.
    // A successful finite value is truncated BEFORE the defined-i32 test,
    // even when its year token is unused by this actual General caller.
    {
        use crate::weather::raw::{SourceDateScratch, process_source_date_string_with_diagnostic};
        let mut date = SourceDateScratch::default();
        let mut failed = false;
        let mut diagnostics = Vec::new();
        process_source_date_string_with_diagnostic(
            "2147483647.9/12/31",
            &mut date,
            false,
            &mut failed,
            &mut |message| {
                diagnostics.push(message.to_owned());
                Ok(())
            },
        )?;
        assert_eq!((date.month, date.day), (Some(12), Some(31)));
        assert!(!failed && diagnostics.is_empty());
    }
    for text in [
        "NANE",
        "INFE",
        "1E9999E",
        "2013/NANE/31",
        "2147483648/13/31",
        "2013/13/2147483648",
    ] {
        use crate::weather::raw::{SourceDateScratch, process_source_date_string_with_diagnostic};
        let mut date = SourceDateScratch::default();
        let mut failed = false;
        let mut diagnostics = Vec::new();
        assert!(
            process_source_date_string_with_diagnostic(
                text,
                &mut date,
                false,
                &mut failed,
                &mut |message| {
                    diagnostics.push(message.to_owned());
                    Ok(())
                }
            )
            .is_err(),
            "{text}"
        );
        assert!(!failed, "{text}");
        assert!(date.month.is_none() && date.day.is_none(), "{text}");
        assert!(diagnostics.is_empty(), "{text}");
    }
    for (mode, first) in [
        ("No", [10.0, 175.0, 175.0, 175.0, 175.0]),
        ("Average", [10.0, 120.0, 175.0, 175.0, 175.0]),
        ("Linear", [10.0, 40.0, 85.0, 130.0, 175.0]),
    ] {
        let raw = authored_raw(fixture, mode, false);
        let mut input = processor(fixture, &raw)?;
        let declarations = schedule_declarations(fixture, &raw)?;
        let mut manager = SelectedScheduleManager::default();
        manager.init_constant_state(0)?;
        let (mut state, mut errors, mut console, mut audit) = (
            interpolation_error_state(),
            Vec::new(),
            Vec::new(),
            MemoryAudit::default(),
        );
        {
            let mut backend = SourceErrorBackend {
                state: &mut state,
                error_stream: Some(&mut errors),
                console: &mut console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            };
            manager.process_schedule_input_with_live_backend(
                &mut input,
                &declarations,
                4,
                &mut audit,
                &mut backend,
                &flags,
            )?;
        }
        let index = manager
            .get_source_schedule("INTERNAL GAIN SCHEDULE")?
            .ok_or("actual authored Compact pointer")?;
        for ordinal in 0..96 {
            let clock = WeatherGlobalState {
                hour_of_day: ordinal / 4 + 1,
                time_step: ordinal % 4 + 1,
                time_steps_in_hour: 4,
                ..Default::default()
            };
            let environment = WeatherEnvironmentState {
                day_of_year_schedule: 1,
                day_of_week: 3,
                ..Default::default()
            };
            manager.update_source_schedule_values(&clock, &environment)?;
            let actual = manager.source_current_schedule_value(index)?;
            let expected: f64 = first.get(ordinal as usize).copied().unwrap_or(175.0);
            assert_eq!(
                actual.to_bits(),
                expected.to_bits(),
                "{mode} actual live lookup {ordinal}"
            );
        }
        // The last live currentVal is175, yet the same retained Detailed graph
        // contains10. This proves hasVal does not substitute the current value.
        let actual_clock = WeatherGlobalState {
            time_steps_in_hour: 4,
            ..Default::default()
        };
        let constructor_clock = WeatherGlobalState {
            time_steps_in_hour: 0,
            ..Default::default()
        };
        assert_eq!(manager.source_current_schedule_value(index)?, 175.0);
        assert!(manager.source_has_value(index, 10.0, &actual_clock)?);
        assert!(!manager.source_has_value(index, -1.0, &actual_clock)?);
        assert!(!manager.source_has_value(index, 10.0, &constructor_clock)?);
        let always_on = manager
            .get_source_schedule("CONSTANT-1.0")?
            .ok_or("actual built-in constant")?;
        assert!(manager.source_has_value(always_on, 1.0, &constructor_clock)?);
        assert_eq!(state.tracking.total_warning_errors, i32::from(mode == "No"));
        assert_eq!(
            state.tracking.total_warning_errors_during_warmup,
            i32::from(mode == "No")
        );
        assert_eq!(state.tracking.total_severe_errors, 0);
        assert!(!state.warmup_flag); // Source cells, not backend ctor, were read.
        assert!(
            std::str::from_utf8(&audit.bytes)?
                .ends_with("  Processing Schedule Input -- Complete\n")
        );
    }
    // Equivalent integral HH/MM spellings feed the same real minute producer.
    // Blank HH and MM are existing operand substrings, not absent directives.
    for untils in [
        ["Until::20", "Until:1D0:15.0", "Until:24:"],
        ["Until :20", "Until1E:15+E0", "Until24.0:0D"],
        ["Until:-0:2E1", "Until:1.0:1.5D1", "Until:2.4E+1:0E"],
        ["Until:1e-9999:20", "Until:1:15", "Until:24:-1D-9999"],
        ["Until:-1D-9999:20", "Until:1:15", "Until:24:1e-9999"],
    ] {
        let mut raw = authored_raw(fixture, "Average", false);
        for (at, text) in [3, 5, 7].into_iter().zip(untils) {
            raw["Schedule:Compact"]["Internal Gain Schedule"]["data"][at]["field"] =
                serde_json::json!(text);
        }
        let mut input = processor(fixture, &raw)?;
        let declarations = schedule_declarations(fixture, &raw)?;
        let mut manager = SelectedScheduleManager::default();
        manager.init_constant_state(0)?;
        manager.process_schedule_input(
            &mut input,
            &declarations,
            4,
            &mut MemoryAudit::default(),
        )?;
        let index = manager
            .get_source_schedule("INTERNAL GAIN SCHEDULE")?
            .ok_or("actual integral Until schedule")?;
        let day = manager
            .day_schedules
            .iter()
            .find(|day| day.name == "INTERNAL GAIN SCHEDULE_dy_1")
            .ok_or("actual integral Until Day owner")?;
        assert_eq!(day.timestep_values.len(), 96);
        for step in 0..96 {
            let expected = match step {
                0 => 10.0_f64,
                1 => 120.0,
                _ => 175.0,
            };
            assert_eq!(day.timestep_values[step].to_bits(), expected.to_bits());
        }
        for step in 0..96 {
            let expected = match step {
                0 => 10.0_f64,
                1 => 120.0,
                _ => 175.0,
            };
            let clock = WeatherGlobalState {
                hour_of_day: step / 4 + 1,
                time_step: step % 4 + 1,
                time_steps_in_hour: 4,
                ..Default::default()
            };
            let environment = WeatherEnvironmentState {
                day_of_year_schedule: 1,
                day_of_week: 3,
                ..Default::default()
            };
            manager.update_source_schedule_values(&clock, &environment)?;
            assert_eq!(
                manager.source_current_schedule_value(index)?.to_bits(),
                expected.to_bits()
            );
        }
    }
    // Keep the real nonIntegral Warning/Continue before the unsupported native
    // string-floating-format operation; no successful population/fatal is made up.
    for (until, label) in [("Until:0.5:20", "HH"), ("Until:00:20.5", "MM")] {
        let mut raw = authored_raw(fixture, "Average", false);
        raw["Schedule:Compact"]["Internal Gain Schedule"]["data"][3]["field"] =
            serde_json::json!(until);
        let mut input = processor(fixture, &raw)?;
        let declarations = schedule_declarations(fixture, &raw)?;
        let mut manager = SelectedScheduleManager::default();
        manager.init_constant_state(0)?;
        let (mut state, mut errors, mut console, mut audit) = (
            interpolation_error_state(),
            Vec::new(),
            Vec::new(),
            MemoryAudit::default(),
        );
        let result = {
            let mut backend = SourceErrorBackend {
                state: &mut state,
                error_stream: Some(&mut errors),
                console: &mut console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            };
            manager.process_schedule_input_with_live_backend(
                &mut input,
                &declarations,
                4,
                &mut audit,
                &mut backend,
                &flags,
            )
        };
        assert!(result.is_err());
        assert_eq!(
            (
                state.tracking.total_warning_errors,
                state.tracking.total_warning_errors_during_warmup
            ),
            (1, 1)
        );
        assert_eq!(state.tracking.total_severe_errors, 0);
        let text = std::str::from_utf8(&errors)?;
        let warning = text
            .find(&format!("non-integer numeric in {label}"))
            .ok_or("real nonIntegral warning")?;
        let continuation = text
            .find("Other errors may result.")
            .ok_or("real nonIntegral continuation")?;
        assert!(warning < continuation);
        assert!(!text.contains("Until value to be used"));
        let day = manager
            .day_schedules
            .iter()
            .find(|day| day.name == "INTERNAL GAIN SCHEDULE_dy_1")
            .ok_or("actual pre-population Day owner")?;
        assert!(
            day.timestep_values
                .iter()
                .all(|value| value.to_bits() == 0.0_f64.to_bits())
        );
    }
    // Both empty events are actual authored intervals, not omitted fields.
    // The real minute child writes1440bins; the real manager writes96samples.
    for (mode, interpolation, source_mode) in [
        (
            "No",
            ep_model::ScheduleInterpolation::No,
            super::minute::SourceScheduleInterpolation::No,
        ),
        (
            "Average",
            ep_model::ScheduleInterpolation::Average,
            super::minute::SourceScheduleInterpolation::Average,
        ),
        (
            "Linear",
            ep_model::ScheduleInterpolation::Linear,
            super::minute::SourceScheduleInterpolation::Linear,
        ),
    ] {
        let untils = ["UNTIL:0:0", "UNTIL:03:00", "UNTIL:12:60", "UNTIL:24:00"].map(str::to_owned);
        let values = [2.0_f64, 0.0, 2.0, 1.0];
        let (mut minutes, mut assigned, mut found_errors) = ([0.0; 1440], [false; 1440], false);
        let mut diagnostics = Vec::new();
        super::minute::process_interval_fields_with(
            &untils,
            &values,
            4,
            4,
            &mut minutes,
            &mut assigned,
            &mut found_errors,
            "ACTUAL EMPTY INTERVALS",
            "Schedule:Compact",
            source_mode,
            15,
            &mut |kind, message| {
                diagnostics.push((kind, message.to_owned()));
                Ok(())
            },
        )?;
        assert!(!found_errors);
        assert!(assigned.iter().all(|value| *value));
        assert_eq!(diagnostics.len(), 2);
        assert!(diagnostics.iter().all(|(kind, message)| *kind
            == super::minute::SourceScheduleMinuteDiagnostic::Warning
            && message.contains("zero time interval detected")));
        if mode != "Linear" {
            assert!(minutes[..180].iter().all(|value| *value == 0.0));
            assert!(minutes[180..].iter().all(|value| *value == 1.0));
        } else {
            assert!(minutes[0] > 1.0 && minutes[180] > 1.0);
        }
        let canonical = [0, 180, 180, 1440];
        let segments = canonical
            .into_iter()
            .zip(values)
            .map(
                |(until_minute_of_day, value)| ep_model::ScheduleCompactSegment {
                    until_minute_of_day,
                    value,
                },
            )
            .collect::<Vec<_>>();
        let (_, expected_steps) =
            ep_model::populate_schedule_day_values(interpolation, &segments, Some(15));
        let mut raw = authored_raw(fixture, mode, false);
        raw["ScheduleTypeLimits"]["Fraction"]["upper_limit_value"] = serde_json::json!(2);
        let mut data = vec![
            serde_json::json!({"field":"Through:12/31"}),
            serde_json::json!({"field":"For:AllDays"}),
            serde_json::json!({"field":format!("Interpolate:{mode}")}),
        ];
        for (until, value) in untils.iter().zip(values) {
            data.extend([
                serde_json::json!({"field":until}),
                serde_json::json!({"field":value}),
            ]);
        }
        raw["Schedule:Compact"]["Internal Gain Schedule"]["data"] = Value::Array(data);
        let mut input = processor(fixture, &raw)?;
        let declarations = schedule_declarations(fixture, &raw)?;
        let mut manager = SelectedScheduleManager::default();
        manager.init_constant_state(0)?;
        let (mut state, mut errors, mut console, mut audit) = (
            interpolation_error_state(),
            Vec::new(),
            Vec::new(),
            MemoryAudit::default(),
        );
        {
            let mut backend = SourceErrorBackend {
                state: &mut state,
                error_stream: Some(&mut errors),
                console: &mut console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            };
            manager.process_schedule_input_with_live_backend(
                &mut input,
                &declarations,
                4,
                &mut audit,
                &mut backend,
                &flags,
            )?;
        }
        assert_eq!(
            (
                state.tracking.total_warning_errors,
                state.tracking.total_warning_errors_during_warmup
            ),
            (2, 2)
        );
        assert_eq!(state.tracking.total_severe_errors, 0);
        let index = manager
            .get_source_schedule("INTERNAL GAIN SCHEDULE")?
            .ok_or("actual empty interval graph")?;
        let day = manager
            .day_schedules
            .iter()
            .find(|day| day.name == "INTERNAL GAIN SCHEDULE_dy_1")
            .ok_or("actual empty interval Day owner")?;
        assert_eq!(day.timestep_values.len(), 96);
        for (step, value) in day.timestep_values.iter().enumerate() {
            assert_eq!(
                value.to_bits(),
                expected_steps[step].to_bits(),
                "{mode} step{step}"
            );
            let raw_expected = if mode == "Average" {
                minutes[step * 15..(step + 1) * 15].iter().sum::<f64>() / 15.0
            } else {
                minutes[(step + 1) * 15 - 1]
            };
            assert_eq!(value.to_bits(), raw_expected.to_bits());
        }
        let maximum = manager.source_max(index)?;
        if mode == "Linear" {
            assert!(maximum > 1.0);
        } else {
            assert_eq!(maximum, 1.0);
            assert_eq!(manager.source_min(index)?, 0.0);
        }
        for step in 0..96 {
            let clock = WeatherGlobalState {
                hour_of_day: step / 4 + 1,
                time_step: step % 4 + 1,
                time_steps_in_hour: 4,
                ..Default::default()
            };
            let environment = WeatherEnvironmentState {
                day_of_year_schedule: 1,
                day_of_week: 3,
                ..Default::default()
            };
            manager.update_source_schedule_values(&clock, &environment)?;
            assert_eq!(
                manager.source_current_schedule_value(index)?.to_bits(),
                expected_steps[step as usize].to_bits()
            );
        }
    }
    // Actual IP distinguishes a present empty item from a missing whole entry.
    // Read the real getter count/alpha/blank, then use the genuine minute graph
    // and live clock for every reported timestep; no profile is seeded here.
    for (entry, expected_alpha, expected) in [
        (serde_json::json!({"field":"1D0"}), "1D0", 1.0_f64),
        (serde_json::json!({"field":"-5D-1"}), "-5D-1", -0.5),
        (serde_json::json!({"field":"0E"}), "0E", 0.0),
        (serde_json::json!({"field":"1e-9999"}), "1E-9999", 0.0),
        (serde_json::json!({"field":"-1e-9999"}), "-1E-9999", -0.0),
        (serde_json::json!({"field":"1D-9999"}), "1D-9999", 0.0),
        (serde_json::json!({"field":"-1D-9999"}), "-1D-9999", -0.0),
        (serde_json::json!({"field":""}), "", 0.0),
        (serde_json::json!({"field":"   "}), "   ", 0.0),
        (serde_json::json!({}), "", 0.0),
        (serde_json::json!({"ignored":{"field":null}}), "", 0.0),
    ] {
        let mut raw = authored_raw(fixture, "No", false);
        raw["ScheduleTypeLimits"]["Fraction"]["lower_limit_value"] = serde_json::json!(-1);
        raw["ScheduleTypeLimits"]["Fraction"]["upper_limit_value"] = serde_json::json!(1);
        raw["Schedule:Compact"]["Internal Gain Schedule"]["data"] = serde_json::json!([
            {"field":"Through:12/31"},{"field":"For:AllDays"},
            {"field":"Until:24:00"},entry,
        ]);
        let mut input = processor(fixture, &raw)?;
        let ordinal = input
            .source_raw_object_names("Schedule:Compact")?
            .iter()
            .position(|name| name.eq_ignore_ascii_case("INTERNAL GAIN SCHEDULE"))
            .ok_or("actual Compact object ordinal")?;
        let args = input.get_object_def_max_args("Schedule:Compact")?;
        let mut locals: super::SourceScheduleGetterLocals = (
            vec![String::new(); args.alphas + 1],
            vec![String::new(); args.alphas + 1],
            vec![String::new(); args.numerics],
            vec![0.0; args.numerics],
            vec![true; args.alphas + 1],
            vec![true; args.numerics],
        );
        let count = SelectedScheduleManager::get_source_compact_object_item(
            &mut input,
            ordinal,
            &mut locals,
            true,
        )?;
        assert_eq!(count, 6); // Name + Type + four real extensible items.
        assert_eq!(locals.0[5], expected_alpha);
        assert_eq!(locals.4[5], expected_alpha.is_empty());
        let declarations = schedule_declarations(fixture, &raw)?;
        let mut manager = SelectedScheduleManager::default();
        manager.init_constant_state(0)?;
        manager.process_schedule_input(
            &mut input,
            &declarations,
            4,
            &mut MemoryAudit::default(),
        )?;
        let index = manager
            .get_source_schedule("INTERNAL GAIN SCHEDULE")?
            .ok_or("actual scalar schedule")?;
        let actual_day = manager
            .day_schedules
            .iter()
            .find(|day| day.name == "INTERNAL GAIN SCHEDULE_dy_1")
            .ok_or("actual scalar Day owner")?;
        assert_eq!(actual_day.timestep_values.len(), 96);
        assert!(
            actual_day
                .timestep_values
                .iter()
                .all(|value| value.to_bits() == expected.to_bits())
        );
        assert_eq!(manager.source_min(index)?.to_bits(), expected.to_bits());
        assert_eq!(manager.source_max(index)?.to_bits(), expected.to_bits());
        for step in 0..96 {
            let clock = WeatherGlobalState {
                hour_of_day: step / 4 + 1,
                time_step: step % 4 + 1,
                time_steps_in_hour: 4,
                ..Default::default()
            };
            let environment = WeatherEnvironmentState {
                day_of_year_schedule: 1,
                day_of_week: 3,
                ..Default::default()
            };
            manager.update_source_schedule_values(&clock, &environment)?;
            assert_eq!(
                manager.source_current_schedule_value(index)?.to_bits(),
                expected.to_bits()
            );
        }
    }
    // These stop at existing unavailable/schema boundaries; no Native severe
    // text, callback or counter is fabricated by the backend-free caller.
    for entry in [
        Some(serde_json::json!({"field":null})),
        Some(serde_json::json!(null)),
        Some(serde_json::json!({"field":"junk"})),
        Some(serde_json::json!({"field":"+1"})),
        Some(serde_json::json!({"field":"nan"})),
        Some(serde_json::json!({"field":"nanE"})),
        Some(serde_json::json!({"field":"1e9999E"})),
        Some(serde_json::json!({"field":"\u{00a0}1"})),
        Some(serde_json::json!({"field":"Through:12/31"})),
        None,
    ] {
        let mut raw = authored_raw(fixture, "No", false);
        let mut data = vec![
            serde_json::json!({"field":"Through:12/31"}),
            serde_json::json!({"field":"For:AllDays"}),
            serde_json::json!({"field":"Until:24:00"}),
        ];
        if let Some(entry) = entry {
            data.push(entry);
        }
        raw["Schedule:Compact"]["Internal Gain Schedule"]["data"] = Value::Array(data);
        let mut input = processor(fixture, &raw)?;
        let declarations = schedule_declarations(fixture, &raw)?;
        let mut manager = SelectedScheduleManager::default();
        manager.init_constant_state(0)?;
        assert!(
            manager
                .process_schedule_input(&mut input, &declarations, 4, &mut MemoryAudit::default())
                .is_err()
        );
    }
    // Compiler-accepted General MonthDay endpoints reach the actual child,
    // including noncanonical plain3 folds. Genuine Day/Week owners are retained.
    for (through, ordinal) in [
        ("Through: DECEMBER 31", 366_i32),
        ("Through: 31 DEC", 366),
        ("Through: 12-31", 366),
        ("Through: 12 31", 366),
        ("Through 2013/12/31", 366),
        ("Through: 12/31/2013", 366),
        ("Through2013-12-31", 366),
        ("Through 2013:12:31", 366),
        ("Through 2013.5/12.9/31.9", 366),
        ("Through 2.013D+3/1.2E+1/3.1D1", 366),
        ("Through 12.9/31.9/2013.5", 366),
        ("Through -2013.5/-12/-31", 366),
        ("Through 2013.5/2.9/29.9", 60),
        ("Through 2013D-1/12", 12),
        ("Through 12.9/31.9", 366),
        ("Through: DEC 31.9", 366),
        ("Through 31.9 DECADE", 366),
        ("Through DECOR 31.9", 366),
        ("Through 31ST DEC", 366),
        ("Through 31.9 OF DECEMBER", 366),
        ("Through DEC IN 31.9", 366),
        ("Through 31STST DEC", 366),
        ("Through 2.9/29.9", 60),
        ("Through 29TH FEBRUARY", 60),
        ("Through12D0:31E0", 366),
        ("Through 12E/31D", 366),
        ("Through -12.9/-31.9", 366),
        ("Through :12.9/31.9", 366),
        ("Through12+E-1", 336),
        ("Through 2013/2/30", 61),
        ("Through 2013/2/BAD", 31),
        ("Through 2013/3/0", 60),
        ("Through 2013/1/366", 366),
        ("Through 2013/2/335", 366),
        ("Through 2013/11/61", 366),
    ] {
        let mut raw = authored_raw(fixture, "Average", false);
        raw["Schedule:Compact"]["Internal Gain Schedule"]["data"][0]["field"] =
            serde_json::json!(through);
        // Actual Raw Source getters already accept these no-colon spellings;
        // exercise the real minute producer, not a typed preview value.
        for (at, text) in [
            (1, "For AllDays"),
            (2, "Interpolate Average"),
            (3, "Until 00:20"),
            (5, "Until 01:15"),
            (7, "Until 24:00"),
        ] {
            raw["Schedule:Compact"]["Internal Gain Schedule"]["data"][at]["field"] =
                serde_json::json!(text);
        }
        if ordinal < 366 {
            let data = raw["Schedule:Compact"]["Internal Gain Schedule"]["data"]
                .as_array_mut()
                .ok_or("actual authored fixed-date fields")?;
            data.extend([
                serde_json::json!({"field":"Through:12/31"}),
                serde_json::json!({"field":"For:AllDays"}),
                serde_json::json!({"field":"Until:24:00"}),
                serde_json::json!({"field":5}),
            ]);
        }
        let mut input = processor(fixture, &raw)?;
        let declarations = schedule_declarations(fixture, &raw)?;
        let mut manager = SelectedScheduleManager::default();
        manager.init_constant_state(0)?;
        manager.process_schedule_input(
            &mut input,
            &declarations,
            4,
            &mut MemoryAudit::default(),
        )?;
        let index = manager
            .get_source_schedule("INTERNAL GAIN SCHEDULE")?
            .ok_or("actual named-month Compact pointer")?;
        let super::SelectedScheduleData::Detailed(detail) = &manager.schedules[index.0].data else {
            return Err("actual named-month Detailed owner".into());
        };
        let week = detail.week_schedules[1].ok_or("actual first-day Week owner")?;
        let endpoint = usize::try_from(ordinal)?;
        assert!(
            detail.week_schedules[1..=endpoint]
                .iter()
                .all(|slot| *slot == Some(week)),
            "{through}"
        );
        if endpoint < 366 {
            let next_week =
                detail.week_schedules[endpoint + 1].ok_or("actual next-period Week owner")?;
            assert_ne!(next_week, week);
            assert!(
                detail.week_schedules[endpoint + 1..]
                    .iter()
                    .all(|slot| *slot == Some(next_week))
            );
        }
        assert!(
            manager.week_schedules[week].day_schedules[1..]
                .iter()
                .all(Option::is_some)
        );
        for (time_step, expected) in [(1, 10.0_f64), (2, 120.0_f64)] {
            let clock = WeatherGlobalState {
                hour_of_day: 1,
                time_step,
                time_steps_in_hour: 4,
                ..Default::default()
            };
            let environment = WeatherEnvironmentState {
                day_of_year_schedule: ordinal,
                day_of_week: 3,
                ..Default::default()
            };
            manager.update_source_schedule_values(&clock, &environment)?;
            assert_eq!(
                manager.source_current_schedule_value(index)?.to_bits(),
                expected.to_bits(),
                "{through}"
            );
        }
        if ordinal < 366 {
            let clock = WeatherGlobalState {
                hour_of_day: 1,
                time_step: 1,
                time_steps_in_hour: 4,
                ..Default::default()
            };
            let environment = WeatherEnvironmentState {
                day_of_year_schedule: ordinal + 1,
                day_of_week: 3,
                ..Default::default()
            };
            manager.update_source_schedule_values(&clock, &environment)?;
            assert_eq!(
                manager.source_current_schedule_value(index)?.to_bits(),
                5.0_f64.to_bits()
            );
        }
    }
    // The real Through caller stops defined-int/checked-ordinal/array failures
    // before any out-of-range schedule pointer. No Native diagnostic is invented.
    for through in [
        "2013/13/2147483648",
        "2013/2147483648/31",
        "2013/2/2147483647",
        "2013/2/336",
        "2013/13/NANE",
        "2013ST/12/31",
    ] {
        let mut raw = authored_raw(fixture, "Average", false);
        raw["Schedule:Compact"]["Internal Gain Schedule"]["data"][0]["field"] =
            serde_json::json!(format!("Through:{through}"));
        let mut input = processor(fixture, &raw)?;
        let declarations = schedule_declarations(fixture, &raw)?;
        let mut manager = SelectedScheduleManager::default();
        manager.init_constant_state(0)?;
        assert!(
            manager
                .process_schedule_input(&mut input, &declarations, 4, &mut MemoryAudit::default())
                .is_err(),
            "{through}"
        );
    }
    // Source already parses every authored empty period into real Week/Day
    // owners. Only the actual annual pointer graph contributes to min/max.
    for (periods, inactive_number, expected_min, expected_max) in [
        (
            vec![("6/30", 0.25_f64), ("6/30", 2.0), ("12/31", 0.75)],
            2,
            0.25,
            0.75,
        ),
        (vec![("12/31", 0.25_f64), ("6/30", 2.0)], 2, 0.25, 0.25),
    ]
    .into_iter()
    .chain(
        [
            "0",
            "0.0",
            "-0",
            ".5",
            "-5D-1",
            "0E",
            "0D",
            "",
            "   ",
            "1e-9999",
            "-1e-9999",
            "1D-9999",
            "-1D-9999",
            "2013/13/31",
            "13/31/2013",
            "BAD/2/2013",
            "BAD/BAD/BAD",
            "12/31/100",
            "+2013/12/31",
            "2013/+12/31",
            "1e9999/12/31",
            "2013/13/2147483647.9",
            "2013/nan/31",
        ]
        .into_iter()
        .map(|zero| (vec![(zero, 2.0), ("12/31", 0.75)], 1, 0.75, 0.75)),
    ) {
        let mut raw = authored_raw(fixture, "No", false);
        raw["ScheduleTypeLimits"]["Fraction"]["upper_limit_value"] = serde_json::json!(1);
        let mut data = Vec::new();
        for (through, value) in periods {
            data.extend([
                serde_json::json!({"field":format!("Through: {through}")}),
                serde_json::json!({"field":"For: AllDays"}),
                serde_json::json!({"field":"Until: 24:00"}),
                serde_json::json!({"field":value}),
            ]);
        }
        raw["Schedule:Compact"]["Internal Gain Schedule"]["data"] = Value::Array(data);
        let mut input = processor(fixture, &raw)?;
        let declarations = schedule_declarations(fixture, &raw)?;
        let mut manager = SelectedScheduleManager::default();
        manager.init_constant_state(0)?;
        manager.process_schedule_input(
            &mut input,
            &declarations,
            4,
            &mut MemoryAudit::default(),
        )?;
        let index = manager
            .get_source_schedule("INTERNAL GAIN SCHEDULE")?
            .ok_or("actual empty-period Detailed pointer")?;
        let super::SelectedScheduleData::Detailed(detail) = &manager.schedules[index.0].data else {
            return Err("actual empty-period Detailed owner".into());
        };
        let inactive_week = manager
            .week_schedules
            .iter()
            .find(|week| week.name == format!("INTERNAL GAIN SCHEDULE_wk_{inactive_number}"))
            .ok_or("actual inactive Week owner")?;
        assert!(inactive_week.is_used);
        assert!(!inactive_week.is_min_max_set);
        assert!(
            detail.week_schedules[1..]
                .iter()
                .all(|slot| slot.is_some() && *slot != Some(inactive_week.number))
        );
        let inactive_day = manager
            .day_schedules
            .iter()
            .find(|day| day.name == format!("INTERNAL GAIN SCHEDULE_dy_{inactive_number}"))
            .ok_or("actual inactive Day owner")?;
        assert!(inactive_day.is_used);
        assert!(!inactive_day.is_min_max_set);
        assert_eq!(inactive_day.timestep_values.len(), 96);
        assert!(
            inactive_day
                .timestep_values
                .iter()
                .all(|value| *value == 2.0)
        );
        assert_eq!(manager.source_min(index)?, expected_min);
        assert_eq!(manager.source_max(index)?, expected_max);
        let clock = WeatherGlobalState {
            hour_of_day: 1,
            time_step: 1,
            time_steps_in_hour: 4,
            ..Default::default()
        };
        for (day_of_year_schedule, expected) in [(1, expected_min), (183, expected_max)] {
            let environment = WeatherEnvironmentState {
                day_of_year_schedule,
                day_of_week: 3,
                ..Default::default()
            };
            manager.update_source_schedule_values(&clock, &environment)?;
            assert_eq!(
                manager.source_current_schedule_value(index)?.to_bits(),
                expected.to_bits()
            );
        }
    }
    // Blank, unresolved and named-unlimited types leave this Detailed
    // schedule uncached. Its real downstream getter must use the live clock,
    // not currentVal, an inferred minutes clock, or an eager input reduction.
    for type_case in ["missing", "blank", "unknown", "unlimited"] {
        let mut raw = authored_raw(fixture, "Average", false);
        let gain = raw["Schedule:Compact"]["Internal Gain Schedule"]
            .as_object_mut()
            .ok_or("actual Compact fields")?;
        match type_case {
            "missing" => {
                gain.remove("schedule_type_limits_name");
            }
            "blank" => {
                gain.insert("schedule_type_limits_name".into(), serde_json::json!(""));
            }
            "unknown" => {
                gain.insert(
                    "schedule_type_limits_name".into(),
                    serde_json::json!("UNKNOWN LIMIT"),
                );
            }
            "unlimited" => {
                let limits = raw["ScheduleTypeLimits"]["Fraction"]
                    .as_object_mut()
                    .ok_or("actual type fields")?;
                limits.remove("lower_limit_value");
                limits.remove("upper_limit_value");
            }
            _ => return Err("unknown authored type case".into()),
        }
        let mut input = processor(fixture, &raw)?;
        let declarations = schedule_declarations(fixture, &raw)?;
        let mut manager = SelectedScheduleManager::default();
        manager.init_constant_state(0)?;
        let (mut state, mut errors, mut console, mut audit) = (
            interpolation_error_state(),
            Vec::new(),
            Vec::new(),
            MemoryAudit::default(),
        );
        {
            let mut backend = SourceErrorBackend {
                state: &mut state,
                error_stream: Some(&mut errors),
                console: &mut console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            };
            manager.process_schedule_input_with_live_backend(
                &mut input,
                &declarations,
                4,
                &mut audit,
                &mut backend,
                &flags,
            )?;
        }
        let index = manager
            .get_source_schedule("INTERNAL GAIN SCHEDULE")?
            .ok_or("actual Compact pointer")?;
        assert_eq!(
            manager.schedules[index.0].schedule_type_number.is_some(),
            type_case == "unlimited"
        );
        assert!(!manager.schedules[index.0].is_min_max_set);
        assert_eq!(manager.source_current_schedule_value(index)?, 0.0);
        let actual_clock = WeatherGlobalState {
            time_steps_in_hour: 4,
            ..Default::default()
        };
        assert!(manager.source_has_value(index, 10.0, &actual_clock)?);
        assert!(!manager.schedules[index.0].is_min_max_set);
        assert!(manager.source_min(index).is_err()); // legacy owner still missing
        assert!(!manager.schedules[index.0].is_min_max_set);
        assert_eq!(manager.source_min_with_clock(index, &actual_clock)?, 10.0);
        assert_eq!(manager.source_max_with_clock(index, &actual_clock)?, 175.0);
        assert!(manager.source_check_min_with_clock(index, 0.0, &actual_clock)?);
        assert!(!manager.source_check_min_max_inclusive(index, 0.0, 4.0, &actual_clock)?);
        for group in 0..4 {
            assert_eq!(
                manager.source_day_group_min_max_with_clock(index, group, &actual_clock)?,
                (10.0, 175.0)
            );
        }
        let expected_warning = i32::from(type_case != "unlimited");
        assert_eq!(state.tracking.total_warning_errors, expected_warning);
        assert_eq!(
            state.tracking.total_warning_errors_during_warmup,
            expected_warning
        );
        assert_eq!(state.tracking.total_severe_errors, 0);
        assert!(!state.warmup_flag);
        let text = std::str::from_utf8(&errors)?;
        if expected_warning == 1 {
            let header = text
                .find("ProcessScheduleInput: Schedule:Compact = INTERNAL GAIN SCHEDULE")
                .ok_or("actual warning header")?;
            let field = text
                .find(if type_case == "unknown" {
                    "Schedule Type Limits Name = UNKNOWN LIMIT, item not found."
                } else {
                    "Schedule Type Limits Name is empty."
                })
                .ok_or("actual field title")?;
            let last = text
                .find("Schedule will not be validated.")
                .ok_or("actual final Continue")?;
            assert!(header < field && field < last);
        } else {
            assert!(text.is_empty());
        }
    }
    // Incomplete but valid masks keep the genuine MissingDaySchedule0
    // pointers. A failed final Continue retains the already populated first
    // Through period and prevents creation of the later period.
    struct MissingDaySink {
        bytes: Vec<u8>,
        fail_final_continue: bool,
    }
    impl io::Write for MissingDaySink {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            if self.fail_final_continue
                && String::from_utf8_lossy(bytes)
                    .contains("Missing day types will have 0.0 as Schedule Values")
            {
                return Err(io::Error::other("actual missing-day final Continue sink"));
            }
            self.bytes.extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    for fail_final_continue in [false, true] {
        let mut raw = authored_raw(fixture, "Average", false);
        raw["Schedule:Compact"]["Internal Gain Schedule"]["data"] = serde_json::json!([
            {"field":"Through: 6/30"}, {"field":"For: Tuesday"},
            {"field":"Interpolate: Average"}, {"field":"Until: 24:00"}, {"field":2},
            {"field":"Through: 12/31"}, {"field":"For: AllDays"},
            {"field":"Interpolate: Average"}, {"field":"Until: 24:00"}, {"field":3}
        ]);
        let mut input = processor(fixture, &raw)?;
        let declarations = schedule_declarations(fixture, &raw)?;
        let mut manager = SelectedScheduleManager::default();
        manager.init_constant_state(0)?;
        let (mut state, mut errors, mut console, mut audit) = (
            interpolation_error_state(),
            MissingDaySink {
                bytes: Vec::new(),
                fail_final_continue,
            },
            Vec::new(),
            MemoryAudit::default(),
        );
        let result = {
            let mut backend = SourceErrorBackend {
                state: &mut state,
                error_stream: Some(&mut errors),
                console: &mut console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            };
            manager.process_schedule_input_with_live_backend(
                &mut input,
                &declarations,
                4,
                &mut audit,
                &mut backend,
                &flags,
            )
        };
        if fail_final_continue {
            assert!(result.is_err());
        } else {
            result?;
        }
        assert_eq!(
            (
                state.tracking.total_warning_errors,
                state.tracking.total_warning_errors_during_warmup,
                state.tracking.total_severe_errors
            ),
            (1, 1, 0)
        );
        assert!(!state.warmup_flag); // Actual borrowed warmup=true was read.
        let index = *manager
            .schedule_map
            .get("INTERNAL GAIN SCHEDULE")
            .ok_or("actual incomplete-mask Detailed pointer")?;
        let super::SelectedScheduleData::Detailed(detail) = &manager.schedules[index.0].data else {
            return Err("actual incomplete-mask Detailed owner".into());
        };
        let first_week = detail.week_schedules[1].ok_or("actual first Through Week")?;
        assert!(
            detail.week_schedules[1..=182]
                .iter()
                .all(|slot| *slot == Some(first_week))
        );
        let week = &manager.week_schedules[first_week];
        let tuesday = week.day_schedules[3].ok_or("actual Tuesday Day")?;
        assert_ne!(tuesday, 0);
        for day_type in 1..13 {
            assert_eq!(
                week.day_schedules[day_type],
                Some(if day_type == 3 { tuesday } else { 0 })
            );
        }
        assert_eq!(manager.day_schedules[0].name, "MissingDaySchedule-0.0");
        assert_eq!(manager.day_schedules[0].timestep_values.len(), 96);
        assert!(
            manager.day_schedules[0]
                .timestep_values
                .iter()
                .all(|v| v.to_bits() == 0.0_f64.to_bits())
        );
        assert!(
            manager.day_schedules[tuesday]
                .timestep_values
                .iter()
                .all(|v| *v == 2.0)
        );
        let diagnostic_rows = std::str::from_utf8(&errors.bytes)?
            .lines()
            .filter(|line| line.contains("**"))
            .collect::<Vec<_>>();
        let expected = [
            "   ** Warning ** ProcessScheduleInput: Schedule:Compact = INTERNAL GAIN SCHEDULE",
            "   **   ~~~   ** has missing day types in Through=6/30",
            "   **   ~~~   ** Last \"For\" field=FOR: TUESDAY",
            "   **   ~~~   ** Missing day types= \"Sunday\", \"Monday\", \"Wednesday\", \"Thursday\", \"Friday\", \"Saturday\", \"Holiday\", \"SummerDesignDay\", \"WinterDesignDay\", \"CustomDay1\", \"CustomDay2\"",
            "   **   ~~~   ** Missing day types will have 0.0 as Schedule Values",
        ];
        assert_eq!(
            diagnostic_rows,
            expected[..if fail_final_continue { 4 } else { 5 }]
        );
        if fail_final_continue {
            assert!(detail.week_schedules[183..].iter().all(Option::is_none));
            assert!(
                !std::str::from_utf8(&audit.bytes)?
                    .contains("Processing Schedule Input -- Complete")
            );
        } else {
            let second =
                detail.week_schedules[183].ok_or("actual continued second Through Week")?;
            assert_ne!(second, first_week);
            assert!(
                detail.week_schedules[183..]
                    .iter()
                    .all(|slot| *slot == Some(second))
            );
            let clock = WeatherGlobalState {
                hour_of_day: 1,
                time_step: 1,
                time_steps_in_hour: 4,
                ..Default::default()
            };
            for (day_of_year_schedule, day_of_week, holiday_index, expected) in [
                (1, 3, 0, 2.0_f64),
                (1, 1, 0, 0.0),
                (1, 3, 8, 0.0),
                (1, 3, 12, 0.0),
                (183, 1, 0, 3.0),
            ] {
                let environment = WeatherEnvironmentState {
                    day_of_year_schedule,
                    day_of_week,
                    holiday_index,
                    ..Default::default()
                };
                manager.update_source_schedule_values(&clock, &environment)?;
                assert_eq!(
                    manager.source_current_schedule_value(index)?.to_bits(),
                    expected.to_bits()
                );
            }
            assert!(
                std::str::from_utf8(&audit.bytes)?
                    .ends_with("  Processing Schedule Input -- Complete\n")
            );
        }
    }
    // Retain AddDetailed/type and the successful Warning/first Continue when
    // the final Continue sink fails; week/day population has not been reached.
    struct FailTypeContinue {
        bytes: Vec<u8>,
    }
    impl io::Write for FailTypeContinue {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            if String::from_utf8_lossy(bytes).contains("Schedule will not be validated.") {
                return Err(io::Error::other("actual Compact type Continue sink"));
            }
            self.bytes.extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    let mut raw = authored_raw(fixture, "Average", false);
    raw["Schedule:Compact"]["Internal Gain Schedule"]
        .as_object_mut()
        .ok_or("actual Compact fields")?
        .remove("schedule_type_limits_name");
    let mut input = processor(fixture, &raw)?;
    let declarations = schedule_declarations(fixture, &raw)?;
    let mut manager = SelectedScheduleManager::default();
    manager.init_constant_state(0)?;
    let (mut state, mut errors, mut console, mut audit) = (
        interpolation_error_state(),
        FailTypeContinue { bytes: Vec::new() },
        Vec::new(),
        MemoryAudit::default(),
    );
    let result = {
        let mut backend = SourceErrorBackend {
            state: &mut state,
            error_stream: Some(&mut errors),
            console: &mut console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        manager.process_schedule_input_with_live_backend(
            &mut input,
            &declarations,
            4,
            &mut audit,
            &mut backend,
            &flags,
        )
    };
    assert!(result.is_err());
    assert_eq!(state.tracking.total_warning_errors, 1);
    assert_eq!(state.tracking.total_warning_errors_during_warmup, 1);
    let index = manager
        .schedule_map
        .get("INTERNAL GAIN SCHEDULE")
        .ok_or("retained Detailed name")?;
    let super::SelectedScheduleData::Detailed(detail) = &manager.schedules[index.0].data else {
        return Err("retained Detailed dynamic type".into());
    };
    assert_eq!(detail.schedule_kind, 1);
    assert!(manager.schedules[index.0].schedule_type_number.is_none());
    assert!(manager.week_schedules.is_empty());
    assert_eq!(manager.day_schedules.len(), 1); // genuine built-in MissingDay only
    let text = std::str::from_utf8(&errors.bytes)?;
    assert!(text.contains("Schedule Type Limits Name is empty."));
    assert!(!text.contains("Schedule will not be validated."));
    assert!(!std::str::from_utf8(&audit.bytes)?.contains("Processing Schedule Input -- Complete"));

    // One real Warning succeeded, then its Continue fails before population.
    struct FailContinue {
        bytes: Vec<u8>,
    }
    impl io::Write for FailContinue {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            if String::from_utf8_lossy(bytes).contains("Other errors may result.") {
                return Err(io::Error::other("authored Compact Continue sink"));
            }
            self.bytes.extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    let raw = authored_raw(fixture, "No", false);
    let mut input = processor(fixture, &raw)?;
    let declarations = schedule_declarations(fixture, &raw)?;
    let mut manager = SelectedScheduleManager::default();
    manager.init_constant_state(0)?;
    let (mut state, mut errors, mut console, mut audit) = (
        interpolation_error_state(),
        FailContinue { bytes: Vec::new() },
        Vec::new(),
        MemoryAudit::default(),
    );
    let result = {
        let mut backend = SourceErrorBackend {
            state: &mut state,
            error_stream: Some(&mut errors),
            console: &mut console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        manager.process_schedule_input_with_live_backend(
            &mut input,
            &declarations,
            4,
            &mut audit,
            &mut backend,
            &flags,
        )
    };
    assert!(result.is_err());
    assert_eq!(
        (
            state.tracking.total_warning_errors,
            state.tracking.total_warning_errors_during_warmup
        ),
        (1, 1)
    );
    let day = manager
        .day_schedules
        .iter()
        .find(|d| d.name == "INTERNAL GAIN SCHEDULE_dy_1")
        .ok_or("actual day constructor")?;
    assert!(
        day.timestep_values
            .iter()
            .all(|v| v.to_bits() == 0.0_f64.to_bits())
    );
    assert_eq!(day.sum_timestep_values.to_bits(), 0.0_f64.to_bits());
    // A completed severe minute child emits its parent Continue, populates the
    // retained values, then reaches the real final fatal after validation.
    let raw = authored_raw(fixture, "No", true);
    let mut input = processor(fixture, &raw)?;
    let declarations = schedule_declarations(fixture, &raw)?;
    let mut manager = SelectedScheduleManager::default();
    manager.init_constant_state(0)?;
    let (mut state, mut errors, mut console, mut audit) = (
        interpolation_error_state(),
        Vec::new(),
        Vec::new(),
        MemoryAudit::default(),
    );
    let result = {
        let mut backend = SourceErrorBackend {
            state: &mut state,
            error_stream: Some(&mut errors),
            console: &mut console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        manager.process_schedule_input_with_live_backend(
            &mut input,
            &declarations,
            4,
            &mut audit,
            &mut backend,
            &flags,
        )
    };
    assert!(result.is_err());
    assert_eq!(
        (
            state.tracking.total_severe_errors,
            state.tracking.total_severe_errors_during_warmup
        ),
        (1, 1)
    );
    let day = manager
        .day_schedules
        .iter()
        .find(|d| d.name == "INTERNAL GAIN SCHEDULE_dy_1")
        .ok_or("actual populated day")?;
    assert!(
        day.timestep_values
            .iter()
            .all(|v| v.to_bits() == 10.0_f64.to_bits())
    );
    assert_eq!(day.sum_timestep_values.to_bits(), 960.0_f64.to_bits());
    let error = std::str::from_utf8(&errors)?;
    assert!(error.contains("ref Schedule:Compact=\"INTERNAL GAIN SCHEDULE\""));
    assert!(error.contains("ProcessScheduleInput: Preceding Errors cause termination."));
    assert!(!std::str::from_utf8(&audit.bytes)?.contains("Processing Schedule Input -- Complete"));
    Ok(())
}

// Extend the same retained-owner test with authored mixed getters and real
// minute/selector children. These values are not a Native-output fixture.
fn verify_authored_interval_list_week_graph(fixture: &Value) -> TestResult<()> {
    use crate::heat_balance::error_backend::{SourceDiagnosticFlags, SourceErrorBackend};
    let week_fields = [
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
    let fixture_for = |raw: &Value| -> TestResult<Value> {
        let mut f = fixture.clone();
        for (family, definition) in [
            (
                "Schedule:Day:Interval",
                serde_json::json!(["Schedule:Day:Interval", 3, 0, 1, 1, "data"]),
            ),
            (
                "Schedule:Day:List",
                serde_json::json!(["Schedule:Day:List", 3, 1, 0, 1, "extensions"]),
            ),
            (
                "Schedule:Week:Daily",
                serde_json::json!(["Schedule:Week:Daily", 13, 0, 0, 0, null]),
            ),
            (
                "Schedule:Week:Compact",
                serde_json::json!(["Schedule:Week:Compact", 1, 0, 2, 0, "data"]),
            ),
            (
                "Schedule:Year",
                serde_json::json!(["Schedule:Year", 2, 0, 1, 4, "schedule_weeks"]),
            ),
        ] {
            f["object_order"][family] =
                serde_json::json!(object(&raw[family])?.keys().collect::<Vec<_>>());
            let definitions = f["definitions"]
                .as_array_mut()
                .ok_or("real query definitions")?;
            if !definitions.iter().any(|r| r[0] == family) {
                definitions.push(definition);
            }
        }
        f["object_order"]["Schedule:Compact"] = serde_json::json!([]);
        Ok(f)
    };
    let make_raw = |mode: &str| {
        let mut raw = fixture["cohorts"][0]["raw"].clone();
        raw["Schedule:Compact"] = serde_json::json!({});
        // Keep strict Day warnings about this producer's rounded Linear end
        // separate from the bounded nominal0..1 physical Native comparison.
        raw["ScheduleTypeLimits"]["Fraction"]["upper_limit_value"] = serde_json::json!(2);
        raw["Schedule:Day:Interval"] = serde_json::json!({"I":{
            "schedule_type_limits_name":"Fraction","interpolate_to_timestep":mode,
            "data":[{"time":"00:20"},{"time":"24D0:0E","value_until_time":1}]
        }});
        let list = (0..144)
            .map(|n| {
                if n == 0 {
                    serde_json::json!({})
                } else {
                    serde_json::json!({"value":n%2})
                }
            })
            .collect::<Vec<_>>();
        raw["Schedule:Day:List"] = serde_json::json!({"L":{"schedule_type_limits_name":"Fraction",
            "interpolate_to_timestep":mode,"minutes_per_item":10,"extensions":list}});
        let daily = Value::Object(
            week_fields
                .iter()
                .map(|f| ((*f).into(), serde_json::json!("L")))
                .collect(),
        );
        raw["Schedule:Week:Daily"] = serde_json::json!({"WL":daily});
        raw["Schedule:Week:Compact"] = serde_json::json!({"W":{"data":[
            {"daytype_list":"For Weekdays","schedule_day_name":"I"},
            {"daytype_list":"Weekends","schedule_day_name":"L"},
            {"daytype_list":"AllOtherDays","schedule_day_name":"I"}
        ]}});
        let year = |week: &str| {
            serde_json::json!({"schedule_type_limits_name":"Fraction","schedule_weeks":[{
            "schedule_week_name":week,"start_month":1,"start_day":1,"end_month":12,"end_day":31}]})
        };
        raw["Schedule:Year"] = serde_json::json!({"Internal Gain Schedule":year("W"),"CON-01 System Availability":year("WL")});
        raw
    };
    let (warmup, sizing, kickoff, developer) = (true, false, false, false);
    let flags = SourceDiagnosticFlags {
        warmup: &warmup,
        doing_sizing: &sizing,
        kickoff: &kickoff,
        developer: &developer,
    };
    for mode in ["No", "Average", "Linear"] {
        let raw = make_raw(mode);
        let f = fixture_for(&raw)?;
        let mut input = processor(&f, &raw)?;
        let mut locals: super::SourceScheduleGetterLocals = (
            vec![String::new(); 20],
            vec![String::new(); 20],
            vec![String::new(); 145],
            vec![0.0; 145],
            vec![true; 20],
            vec![true; 145],
        );
        assert_eq!(
            SelectedScheduleManager::get_source_regular_object_item(
                &mut input,
                "Schedule:Day:Interval",
                0,
                true,
                &mut locals
            )?,
            (5, 2)
        );
        assert_eq!(locals.0[3], "00:20");
        assert_eq!(locals.2[0], "value_until_time");
        assert_eq!(locals.3[0], 0.0);
        assert!(locals.5[0]);
        assert_eq!(
            SelectedScheduleManager::get_source_regular_object_item(
                &mut input,
                "Schedule:Day:List",
                0,
                true,
                &mut locals
            )?,
            (3, 145)
        );
        assert_eq!(locals.3[0], 10.0);
        assert!(!locals.5[0]);
        assert_eq!(locals.3[1], 0.0);
        assert!(locals.5[1]);
        assert!(!locals.5[2]);
        let mut declarations = schedule_declarations(&f, &raw)?;
        declarations.source_is_epjson = Some(true);
        let mut manager = SelectedScheduleManager::default();
        manager.init_constant_state(0)?;
        let (mut state, mut errors, mut console, mut audit) = (
            interpolation_error_state(),
            Vec::new(),
            Vec::new(),
            MemoryAudit::default(),
        );
        {
            let mut backend = SourceErrorBackend {
                state: &mut state,
                error_stream: Some(&mut errors),
                console: &mut console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            };
            manager.process_schedule_input_with_live_backend(
                &mut input,
                &declarations,
                4,
                &mut audit,
                &mut backend,
                &flags,
            )?;
        }
        let i = *manager
            .day_schedule_map
            .get("I")
            .ok_or("real Interval Day")?;
        let l = *manager.day_schedule_map.get("L").ok_or("real List Day")?;
        assert_eq!((i, l), (1, 2));
        assert!(manager.day_schedules[i].is_used && manager.day_schedules[l].is_used);
        let w = *manager
            .week_schedule_map
            .get("W")
            .ok_or("real Compact Week")?;
        assert_eq!(
            (
                *manager
                    .week_schedule_map
                    .get("WL")
                    .ok_or("real Daily Week")?,
                w
            ),
            (0, 1)
        );
        assert_eq!(manager.week_schedules[w].day_schedules[0], None);
        for day_type in 1..13 {
            assert_eq!(
                manager.week_schedules[w].day_schedules[day_type],
                Some(if day_type == 1 || day_type == 7 { l } else { i })
            );
        }
        let mut expected_i = [0.0_f64; 1440];
        if mode == "Linear" {
            let mut current = 0.0;
            for value in &mut expected_i[20..] {
                current += 1.0 / 1420.0;
                *value = current;
            }
        } else {
            expected_i[20..].fill(1.0);
        }
        let expected_l = (0..1440)
            .map(|minute| ((minute / 10) % 2) as f64)
            .collect::<Vec<_>>();
        for (day, minutes) in [(i, &expected_i[..]), (l, &expected_l[..])] {
            let expected = (0..96)
                .map(|at| {
                    if mode == "Average" {
                        minutes[at * 15..at * 15 + 15].iter().sum::<f64>() / 15.0
                    } else {
                        minutes[at * 15 + 14]
                    }
                })
                .collect::<Vec<_>>();
            assert_eq!(
                manager.day_schedules[day]
                    .timestep_values
                    .iter()
                    .map(|v| v.to_bits())
                    .collect::<Vec<_>>(),
                expected.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                "{mode} actual minute producer"
            );
            assert_eq!(
                manager.day_schedules[day].sum_timestep_values.to_bits(),
                expected.iter().sum::<f64>().to_bits()
            );
            if day == l && mode == "Linear" {
                assert_eq!(expected[0..4], [1.0, 0.0, 0.0, 1.0]);
            }
        }
        let gain = manager
            .get_source_schedule("INTERNAL GAIN SCHEDULE")?
            .ok_or("actual Year consumer")?;
        for step in 0..96 {
            let clock = WeatherGlobalState {
                hour_of_day: step / 4 + 1,
                time_step: step % 4 + 1,
                time_steps_in_hour: 4,
                ..Default::default()
            };
            let environment = WeatherEnvironmentState {
                day_of_year_schedule: 60,
                day_of_week: 3,
                ..Default::default()
            };
            manager.update_source_schedule_values(&clock, &environment)?;
            assert_eq!(
                manager.source_current_schedule_value(gain)?.to_bits(),
                manager.day_schedules[i].timestep_values[step as usize].to_bits()
            );
        }
        assert_eq!(
            state.tracking.total_warning_errors,
            if mode == "No" { 1 } else { 0 }
        );
        assert_eq!(
            state.tracking.total_warning_errors_during_warmup,
            if mode == "No" { 1 } else { 0 }
        );
        assert_eq!(state.tracking.total_severe_errors, 0);
        if mode == "No" {
            assert!(
                std::str::from_utf8(&errors)?
                    .contains("not a multiple of the minutes for each timestep: 00:20")
            );
        }
    }
    // Stateful real empty intervals preserve unused2; a live borrower
    // completes the exact two whole-hour empty-interval warnings.
    let mut raw = make_raw("No");
    raw["Schedule:Day:Interval"]["I"]["data"] = serde_json::json!([
        {"time":"0:0","value_until_time":2},{"time":"03D0:"},
        {"time":"12:60","value_until_time":2},{"time":"24:","value_until_time":1}]);
    let f = fixture_for(&raw)?;
    let mut input = processor(&f, &raw)?;
    let mut declarations = schedule_declarations(&f, &raw)?;
    declarations.source_is_epjson = Some(true);
    let mut manager = SelectedScheduleManager::default();
    manager.init_constant_state(0)?;
    let (mut state, mut errors, mut console) =
        (interpolation_error_state(), Vec::new(), Vec::new());
    {
        let mut backend = SourceErrorBackend {
            state: &mut state,
            error_stream: Some(&mut errors),
            console: &mut console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        manager.process_schedule_input_with_live_backend(
            &mut input,
            &declarations,
            4,
            &mut MemoryAudit::default(),
            &mut backend,
            &flags,
        )?;
    }
    let i = *manager
        .day_schedule_map
        .get("I")
        .ok_or("actual empty Interval")?;
    assert_eq!(&manager.day_schedules[i].timestep_values[..12], &[0.0; 12]);
    assert!(
        manager.day_schedules[i].timestep_values[12..]
            .iter()
            .all(|v| *v == 1.0)
    );
    assert_eq!(manager.day_schedules[i].sum_timestep_values, 84.0);
    assert_eq!(
        (
            state.tracking.total_warning_errors,
            state.tracking.total_warning_errors_during_warmup
        ),
        (2, 2)
    );
    assert_eq!(
        std::str::from_utf8(&errors)?
            .matches("zero time interval detected, Schedule:Day:Interval=I")
            .count(),
        2
    );
    // A selected Day is genuinely marked used before a malformed selector;
    // rejected selector effects do not manufacture a successful Week pointer.
    for selector in ["Monday Weekdays", "Funday"] {
        let mut raw = make_raw("No");
        raw["Schedule:Day:Interval"]["I"]["data"] = serde_json::json!([{"time":"24:00"}]);
        raw["Schedule:Week:Compact"]["W"]["data"] =
            serde_json::json!([{"daytype_list":selector,"schedule_day_name":"I"}]);
        let f = fixture_for(&raw)?;
        let mut input = processor(&f, &raw)?;
        let mut declarations = schedule_declarations(&f, &raw)?;
        declarations.source_is_epjson = Some(true);
        let mut manager = SelectedScheduleManager::default();
        manager.init_constant_state(0)?;
        assert!(
            manager
                .process_schedule_input(&mut input, &declarations, 4, &mut MemoryAudit::default())
                .is_err()
        );
        let i = *manager
            .day_schedule_map
            .get("I")
            .ok_or("actual selector Day")?;
        assert!(manager.day_schedules[i].is_used);
        let w = *manager
            .week_schedule_map
            .get("W")
            .ok_or("actual selector Week")?;
        assert!(
            manager.week_schedules[w].day_schedules[1..]
                .iter()
                .all(|v| *v == Some(0))
        );
    }
    // List's independent strict range and integer checks use its real 'for'
    // wording; permit the Year root itself so these remain warnings only.
    let mut raw = make_raw("No");
    raw["ScheduleTypeLimits"]["Fraction"]["numeric_type"] = serde_json::json!("Discrete");
    raw["ScheduleTypeLimits"]["Fraction"]["upper_limit_value"] = serde_json::json!(1);
    raw["Schedule:Day:Interval"]["I"]["data"] = serde_json::json!([{"time":"24:00"}]);
    // No samples the ending minute14, which belongs to the second10-minute item.
    raw["Schedule:Day:List"]["L"]["extensions"][1] = serde_json::json!({"value":1.5});
    for name in ["Internal Gain Schedule", "CON-01 System Availability"] {
        raw["Schedule:Year"][name]["schedule_type_limits_name"] = serde_json::json!("Control Type");
    }
    let f = fixture_for(&raw)?;
    let mut input = processor(&f, &raw)?;
    let mut declarations = schedule_declarations(&f, &raw)?;
    declarations.source_is_epjson = Some(true);
    let mut manager = SelectedScheduleManager::default();
    manager.init_constant_state(0)?;
    let (mut state, mut errors, mut console) =
        (interpolation_error_state(), Vec::new(), Vec::new());
    {
        let mut backend = SourceErrorBackend {
            state: &mut state,
            error_stream: Some(&mut errors),
            console: &mut console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        manager.process_schedule_input_with_live_backend(
            &mut input,
            &declarations,
            4,
            &mut MemoryAudit::default(),
            &mut backend,
            &flags,
        )?;
    }
    assert_eq!(
        (
            state.tracking.total_warning_errors,
            state.tracking.total_warning_errors_during_warmup
        ),
        (2, 2)
    );
    assert_eq!(state.tracking.total_severe_errors, 0);
    let text = std::str::from_utf8(&errors)?;
    let range = text
        .find("Values are outside of range for schedule_type_limits_name=FRACTION")
        .ok_or("actual List range Warning")?;
    let integer = text
        .find("One or more values are not integer for schedule_type_limits_name=FRACTION")
        .ok_or("actual List integer Warning")?;
    assert!(range < integer);
    // A completed incomplete-day minute child must carry its actual Severe
    // effects through later owner construction to the caller's genuine Fatal.
    let mut raw = make_raw("No");
    raw["Schedule:Day:Interval"]["I"]["data"] =
        serde_json::json!([{"time":"23:00","value_until_time":1}]);
    let f = fixture_for(&raw)?;
    let mut input = processor(&f, &raw)?;
    let mut declarations = schedule_declarations(&f, &raw)?;
    declarations.source_is_epjson = Some(true);
    let mut manager = SelectedScheduleManager::default();
    manager.init_constant_state(0)?;
    let (mut state, mut errors, mut console, mut audit) = (
        interpolation_error_state(),
        Vec::new(),
        Vec::new(),
        MemoryAudit::default(),
    );
    let result = {
        let mut backend = SourceErrorBackend {
            state: &mut state,
            error_stream: Some(&mut errors),
            console: &mut console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        manager.process_schedule_input_with_live_backend(
            &mut input,
            &declarations,
            4,
            &mut audit,
            &mut backend,
            &flags,
        )
    };
    assert!(result.is_err());
    assert_eq!(
        (
            state.tracking.total_severe_errors,
            state.tracking.total_severe_errors_during_warmup
        ),
        (60, 60)
    );
    let text = std::str::from_utf8(&errors)?;
    assert_eq!(
        text.lines()
            .filter(|line| line.contains("** Severe  **")
                && line.contains("incomplete day detected, Schedule:Day:Interval=I"))
            .count(),
        60
    );
    assert!(text.contains("ProcessScheduleInput: Preceding Errors cause termination."));
    assert!(!std::str::from_utf8(&audit.bytes)?.contains("Processing Schedule Input -- Complete"));
    let i = *manager
        .day_schedule_map
        .get("I")
        .ok_or("actual incomplete Interval Day")?;
    assert!(
        manager.day_schedules[i].timestep_values[..92]
            .iter()
            .all(|v| *v == 1.0)
    );
    assert_eq!(&manager.day_schedules[i].timestep_values[92..], &[0.0; 4]);
    assert!(manager.schedule_map.contains_key("INTERNAL GAIN SCHEDULE"));
    Ok(())
}

// Authored raw/IP owners extend the same existing schedule cohort. These EIO
// bytes are emitted by the real report stage, not a stored Native report seed.
fn verify_authored_schedule_input_reports(fixture: &Value) -> TestResult<()> {
    use crate::heat_balance::error_backend::{SourceDiagnosticFlags, SourceErrorBackend};
    use crate::heat_balance::source_initialization::ScheduleInputFileWriters;

    #[derive(Default)]
    struct ReportStream {
        bytes: Vec<u8>,
        fail_at: Option<usize>,
    }
    impl io::Write for ReportStream {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            let count = self.fail_at.map_or(bytes.len(), |end| {
                end.saturating_sub(self.bytes.len()).min(bytes.len())
            });
            if count == 0 && !bytes.is_empty() {
                return Err(io::Error::other("authored actual EIO write failure"));
            }
            self.bytes.extend_from_slice(&bytes[..count]);
            Ok(count)
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    #[derive(Default)]
    struct ReportFiles {
        audit: Vec<u8>,
        eio: ReportStream,
        opens: usize,
    }
    impl ScheduleAuditFileHost for ReportFiles {
        fn audit_path(&self) -> &Path {
            Path::new("authored-schedule-report.audit")
        }
        fn ensure_audit_open(&mut self) -> io::Result<&mut dyn io::Write> {
            self.opens += 1;
            Ok(&mut self.audit)
        }
        fn ensure_schedule_input_writers(&mut self) -> io::Result<ScheduleInputFileWriters<'_>> {
            self.opens += 1;
            Ok(ScheduleInputFileWriters {
                audit: &mut self.audit,
                eio: Some(&mut self.eio),
            })
        }
    }
    let clock = WeatherGlobalState {
        time_steps_in_hour: 4,
        minutes_in_time_step: 15,
        warmup_flag: true,
        ..Default::default()
    };
    let (kickoff, developer, sizing) = (false, false, false);
    let flags = SourceDiagnosticFlags {
        kickoff: &kickoff,
        developer: &developer,
        warmup: &clock.warmup_flag,
        doing_sizing: &sizing,
    };
    let week_fields = [
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
    let make_raw = |mode: &str| {
        let mut raw = authored_raw(fixture, mode, false);
        raw["ScheduleTypeLimits"]["Discrete Report"] = serde_json::json!({
            "lower_limit_value":2.9,"upper_limit_value":4.9,"numeric_type":"Discrete"
        });
        raw["ScheduleTypeLimits"]["Unlimited Report"] = serde_json::json!({});
        let mut hourly = Value::Object(
            (1..=24)
                .map(|hour| (format!("hour_{hour}"), serde_json::json!(0.25)))
                .collect(),
        );
        hourly["schedule_type_limits_name"] = serde_json::json!("Fraction");
        // Equal scalar profiles still have distinct genuine Day/Week identities.
        raw["Schedule:Day:Hourly"] = serde_json::json!({"H":hourly.clone(), "D":hourly});
        let week = |day: &str| {
            Value::Object(
                week_fields
                    .iter()
                    .map(|name| ((*name).into(), serde_json::json!(day)))
                    .collect(),
            )
        };
        raw["Schedule:Week:Daily"] = serde_json::json!({"W":week("H"),"V":week("D")});
        let period = |week: &str, sm: i32, sd: i32, em: i32, ed: i32| {
            serde_json::json!({
                "schedule_week_name":week,"start_month":sm,"start_day":sd,"end_month":em,"end_day":ed
            })
        };
        raw["Schedule:Year"] = serde_json::json!({"Y":{"schedule_type_limits_name":"Fraction","schedule_weeks":[
            period("W",1,1,1,31), period("V",2,1,2,1), period("W",2,2,12,31)
        ]}});
        raw["Output:Schedules"] = serde_json::json!({
            " ":{"key_field":"Timestep"},
            "":{"key_field":"Hourly","unrelated_valid_metadata":{"kept":true}},
            "again":{"key_field":"Hourly"}
        });
        raw
    };
    let fixture_for = |raw: &Value| -> TestResult<Value> {
        let mut f = fixture.clone();
        let type_order = f["object_order"]["ScheduleTypeLimits"]
            .as_array_mut()
            .ok_or("actual type object order")?;
        type_order.extend([
            serde_json::json!("Discrete Report"),
            serde_json::json!("Unlimited Report"),
        ]);
        for (family, order, definition) in [
            (
                "Schedule:Day:Hourly",
                vec!["H", "D"],
                serde_json::json!(["Schedule:Day:Hourly", 2, 24, 0, 0, null]),
            ),
            (
                "Schedule:Week:Daily",
                vec!["W", "V"],
                serde_json::json!(["Schedule:Week:Daily", 13, 0, 0, 0, null]),
            ),
            (
                "Schedule:Year",
                vec!["Y"],
                serde_json::json!(["Schedule:Year", 2, 0, 1, 4, "schedule_weeks"]),
            ),
            (
                "Output:Schedules",
                vec!["", " ", "again"],
                serde_json::json!(["Output:Schedules", 1, 0, 0, 0, null]),
            ),
        ] {
            assert_eq!(object(&raw[family])?.len(), order.len());
            f["object_order"][family] = serde_json::json!(order);
            let definitions = f["definitions"]
                .as_array_mut()
                .ok_or("actual report query definitions")?;
            if !definitions.iter().any(|row| row[0] == family) {
                definitions.push(definition);
            }
        }
        Ok(f)
    };
    let mut completed = Vec::new();
    for (mode, first_hour) in [("No", 175.0), ("Average", 175.0), ("Linear", 130.0)] {
        let raw = make_raw(mode);
        let f = fixture_for(&raw)?;
        let mut input = processor(&f, &raw)?;
        let mut declarations = schedule_declarations(&f, &raw)?;
        declarations.source_is_epjson = Some(true);
        let mut manager = SelectedScheduleManager::default();
        manager.init_constant_state(0)?;
        let (mut state, mut errors, mut console, mut files) = (
            interpolation_error_state(),
            Vec::new(),
            Vec::new(),
            ReportFiles::default(),
        );
        {
            let mut backend = SourceErrorBackend {
                state: &mut state,
                error_stream: Some(&mut errors),
                console: &mut console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            };
            manager.process_schedule_input_with_live_backend_and_clock(
                &mut input,
                &declarations,
                4,
                &mut files,
                &mut backend,
                &flags,
                &clock,
            )?;
        }
        assert_eq!(files.opens, 1);
        assert_eq!(
            std::str::from_utf8(&files.audit)?,
            "  Processing Schedule Input -- Start\n  Processing Schedule Input -- Complete\n"
        );
        for name in ["", " ", "again"] {
            assert!(!input.is_object_unused("Output:Schedules", name));
        }
        let text = std::str::from_utf8(&files.eio.bytes)?;
        assert_eq!(text.matches("! <ScheduleTypeLimits>,").count(), 1);
        assert!(text.contains("ScheduleTypeLimits,CONTROL TYPE,Yes,0,4,No\n"));
        assert!(text.contains("ScheduleTypeLimits,FRACTION,Yes,0.00,200.00,Yes\n"));
        assert!(text.contains("ScheduleTypeLimits,DISCRETE REPORT,Yes,2,4,No\n"));
        assert!(text.contains("ScheduleTypeLimits,UNLIMITED REPORT,No,N/A,N/A,N/A\n"));
        let hourly = text
            .find("! Schedule Details Report=Hourly")
            .ok_or("actual Hourly separator")?;
        let timestep = text
            .find("! Schedule Details Report=Timestep")
            .ok_or("actual Timestep separator")?;
        assert!(hourly < timestep);
        assert_eq!(text.matches("! Schedule Details Report=").count(), 2);
        assert!(text.contains("Time (HH:MM) =>,00:15,00:30,00:45,01:00,"));
        assert!(text.contains("Time (HH:MM) =>,01:00,02:00,"));
        let row_values = |level: &str, name: &str| -> TestResult<Vec<f64>> {
            let prefix = format!("DaySchedule - {level},{name},");
            let row = text
                .lines()
                .find(|line| line.starts_with(&prefix))
                .ok_or("actual reported Day row")?;
            let fields = row.split(',').collect::<Vec<_>>();
            assert_eq!(fields[4], "Values:");
            fields[5..].iter().map(|value| Ok(value.parse()?)).collect()
        };
        let gain_day = manager
            .day_schedules
            .iter()
            .find(|day| day.name.starts_with("INTERNAL GAIN SCHEDULE"))
            .ok_or("actual generated gain Day")?;
        let hours = row_values("Hourly", &gain_day.name)?;
        let steps = row_values("Timestep", &gain_day.name)?;
        assert_eq!((hours.len(), steps.len()), (24, 96));
        assert_eq!(hours[0], first_hour);
        for hour in 0..24 {
            assert_eq!(hours[hour], steps[hour * 4 + 3]);
        }
        assert_eq!(
            row_values("Timestep", "MissingDaySchedule-0.0")?,
            vec![0.0; 96]
        );
        assert!(text.contains("DaySchedule - Hourly,MissingDaySchedule-0.0,,No,Values:"));
        for (level, count) in [("Hourly", 24), ("Timestep", 96)] {
            assert_eq!(
                text.lines()
                    .filter(|line| line.starts_with(&format!("DaySchedule - {level},")))
                    .count(),
                manager.day_schedules.len()
            );
            assert_eq!(
                text.lines()
                    .filter(|line| line.starts_with(&format!("WeekSchedule - {level},")))
                    .count(),
                manager.week_schedules.len()
            );
            for week in &manager.week_schedules {
                let prefix = format!("WeekSchedule - {level},{},", week.name);
                let row = text
                    .lines()
                    .find(|line| line.starts_with(&prefix))
                    .ok_or("actual Week report row")?;
                assert_eq!(row.split(',').count(), 14);
            }
            assert_eq!(row_values(level, "H")?, vec![0.25; count]);
            assert!(text.contains(&format!(
                "Schedule - {level},Y,FRACTION,Through Jan 31,W,Through Feb 00,V,Through Dec 31,W\n"
            )));
            for schedule in &manager.schedules {
                if matches!(&schedule.data, super::SelectedScheduleData::Constant { .. }) {
                    assert!(!text.lines().any(|line| {
                        line.starts_with(&format!("Schedule - {level},{},", schedule.name))
                    }));
                }
            }
        }
        let error_text = std::str::from_utf8(&errors)?;
        assert_eq!(
            error_text
                .matches("ProcessScheduleInput: Output:Schedules = HOURLY")
                .count(),
            1
        );
        assert!(error_text.contains("Report level Hourly has already been processed. This report level will not be processed again."));
        let expected = 1 + i32::from(mode == "No");
        assert_eq!(
            (
                state.tracking.total_warning_errors,
                state.tracking.total_warning_errors_during_warmup
            ),
            (expected, expected)
        );
        assert_eq!(state.tracking.total_severe_errors, 0);
        assert!(!state.warmup_flag);
        completed.push((mode, files.eio.bytes));
    }
    // Seven genuine Year ranges preserve the schema-valid folded month/day
    // operands and produce distinct Week pointer runs despite identical Days.
    // This stays in the existing owner body and reaches the same real EIO.
    {
        let mut raw = make_raw("Average");
        let mut f = fixture_for(&raw)?;
        raw["Output:Schedules"]
            .as_object_mut()
            .ok_or("actual Output schedules map")?
            .remove("again");
        f["object_order"]["Output:Schedules"] = serde_json::json!(["", " "]);
        raw["Schedule:Week:Daily"]["V"] = raw["Schedule:Week:Daily"]["W"].clone();
        let ranges = [
            ("W", 1, 1, 2, 30),
            ("V", 3, 2, 2, 31),
            ("W", 3, 3, 4, 31),
            ("V", 5, 2, 6, 31),
            ("W", 7, 2, 9, 31),
            ("V", 10, 2, 11, 31),
            ("W", 12, 2, 12, 31),
        ];
        raw["Schedule:Year"]["Y"]["schedule_weeks"] = serde_json::json!(ranges.into_iter().map(|(week,sm,sd,em,ed)| {
            serde_json::json!({"schedule_week_name":week,"start_month":sm,"start_day":sd,"end_month":em,"end_day":ed})
        }).collect::<Vec<_>>());
        let mut input = processor(&f, &raw)?;
        let mut declarations = schedule_declarations(&f, &raw)?;
        declarations.source_is_epjson = Some(true);
        let mut manager = SelectedScheduleManager::default();
        manager.init_constant_state(0)?;
        let (mut state, mut errors, mut console, mut files) = (
            interpolation_error_state(),
            Vec::new(),
            Vec::new(),
            ReportFiles::default(),
        );
        {
            let mut backend = SourceErrorBackend {
                state: &mut state,
                error_stream: Some(&mut errors),
                console: &mut console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            };
            manager.process_schedule_input_with_live_backend_and_clock(
                &mut input,
                &declarations,
                4,
                &mut files,
                &mut backend,
                &flags,
                &clock,
            )?;
        }
        let w = *manager.week_schedule_map.get("W").ok_or("actual W Week")?;
        let v = *manager.week_schedule_map.get("V").ok_or("actual V Week")?;
        assert_ne!(w, v);
        assert_eq!(
            manager.week_schedules[w].day_schedules,
            manager.week_schedules[v].day_schedules
        );
        let y = manager
            .get_source_schedule("Y")?
            .ok_or("actual folded Year")?;
        let super::SelectedScheduleData::Detailed(detail) = &manager.schedules[y.0].data else {
            return Err(invalid("actual folded Year Detailed owner").into());
        };
        assert_eq!(detail.week_schedules[0], None);
        let ends = [61, 62, 122, 183, 275, 336, 366];
        for day in 1..=366 {
            let range = ends
                .iter()
                .position(|end| day <= *end)
                .ok_or("actual full Year extent")?;
            let expected = if range % 2 == 0 { w } else { v };
            assert_eq!(
                detail.week_schedules[day],
                Some(expected),
                "actual folded Year day{day}"
            );
        }
        let eio = std::str::from_utf8(&files.eio.bytes)?;
        let through = "Through Mar 01,W,Through Mar 02,V,Through May 01,W,Through Jul 01,V,Through Oct 01,W,Through Dec 01,V,Through Dec 31,W";
        for level in ["Hourly", "Timestep"] {
            let prefix = format!("Schedule - {level},Y,FRACTION,");
            let actual = eio
                .lines()
                .filter(|line| line.starts_with(&prefix))
                .collect::<Vec<_>>();
            let expected = format!("{prefix}{through}");
            assert_eq!(actual, vec![expected.as_str()]);
        }
        assert_eq!(files.opens, 1);
        assert_eq!(
            std::str::from_utf8(&files.audit)?,
            "  Processing Schedule Input -- Start\n  Processing Schedule Input -- Complete\n"
        );
        assert_eq!(
            (
                state.tracking.total_warning_errors,
                state.tracking.total_severe_errors
            ),
            (0, 0)
        );
        assert_eq!(
            (
                state.tracking.total_warning_errors_during_warmup,
                state.tracking.total_severe_errors_during_warmup
            ),
            (0, 0)
        );
        assert!(errors.is_empty());
    }
    // A real partial EIO write retains exactly its completed prefix. Processing
    // owners and usage before that failure remain; Audit Complete is not emitted.
    let raw = make_raw("Average");
    let f = fixture_for(&raw)?;
    let mut input = processor(&f, &raw)?;
    let mut declarations = schedule_declarations(&f, &raw)?;
    declarations.source_is_epjson = Some(true);
    let mut manager = SelectedScheduleManager::default();
    manager.init_constant_state(0)?;
    let expected = completed
        .iter()
        .find(|(mode, _)| *mode == "Average")
        .ok_or("actual completed report")?;
    let stop = std::str::from_utf8(&expected.1)?
        .find("DaySchedule - Hourly,")
        .ok_or("actual Day row prefix")?
        + 30;
    let mut files = ReportFiles {
        eio: ReportStream {
            fail_at: Some(stop),
            ..Default::default()
        },
        ..Default::default()
    };
    let (mut state, mut errors, mut console) =
        (interpolation_error_state(), Vec::new(), Vec::new());
    let result = {
        let mut backend = SourceErrorBackend {
            state: &mut state,
            error_stream: Some(&mut errors),
            console: &mut console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        manager.process_schedule_input_with_live_backend_and_clock(
            &mut input,
            &declarations,
            4,
            &mut files,
            &mut backend,
            &flags,
            &clock,
        )
    };
    assert!(matches!(
        result,
        Err(super::SelectedScheduleError::Report(_))
    ));
    assert_eq!(files.eio.bytes, expected.1[..stop]);
    assert_eq!(files.opens, 1);
    assert_eq!(
        std::str::from_utf8(&files.audit)?,
        "  Processing Schedule Input -- Start\n"
    );
    assert!(!input.is_object_unused("Output:Schedules", ""));
    assert!(input.is_object_unused("Output:Schedules", " "));
    assert!(input.is_object_unused("Output:Schedules", "again"));
    assert_eq!(state.tracking.total_warning_errors, 0);
    assert!(manager.get_source_schedule("Y")?.is_some());
    let mut input = processor(&f, &raw)?;
    let mut manager = SelectedScheduleManager::default();
    manager.init_constant_state(0)?;
    assert!(matches!(
        manager.process_schedule_input(&mut input, &declarations, 4, &mut MemoryAudit::default()),
        Err(super::SelectedScheduleError::Unavailable(
            "actual schedule report EIO stream"
        ))
    ));
    Ok(())
}
