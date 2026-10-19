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
    Ok(object(value)?.get(name).ok_or_else(|| invalid("fixture missing member"))?)
}
fn usize_value(value: &Value) -> TestResult<usize> {
    Ok(usize::try_from(value.as_u64().ok_or_else(|| invalid("fixture unsigned integer"))?)?)
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
            v.iter().map(|(k, v)| Ok((k.clone(), raw_value(v)?)))
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
    array(member(member(fixture, "object_order")?, family)?)?.iter()
        .map(|v| Ok(string(v)?.to_owned())).collect()
}
fn processor(fixture: &Value, raw: &Value) -> TestResult<SelectedInputProcessor> {
    let mut definitions = Vec::new();
    for row in array(member(fixture, "definitions")?)? {
        let row = array(row)?;
        if row.len() != 6 { return Err(invalid("query shape width").into()); }
        definitions.push(SourceObjectDefinition {
            object_type: string(&row[0])?.to_owned(),
            alpha_fields: usize_value(&row[1])?, numeric_fields: usize_value(&row[2])?,
            alpha_extensions: usize_value(&row[3])?, numeric_extensions: usize_value(&row[4])?,
            extension: if row[5].is_null() { None } else { Some(string(&row[5])?.to_owned()) },
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
                field_sizes: object(fields)?.iter().map(|(k, v)| (k.clone(), json_size(v))).collect(),
            });
            retained.insert(name.clone(), object(fields)?.iter()
                .map(|(k, v)| Ok((k.clone(), raw_value(v)?)))
                .collect::<TestResult<BTreeMap<_, _>>>()?);
        }
        families.insert(family.clone(), SourceInputFamily::Objects(shapes));
        statistics.objects.insert(family.clone(), retained);
        statistics.object_order.insert(family.clone(), names(fixture, family)?);
    }
    let mut input = SelectedInputProcessor::from_admitted_input(definitions, families)?;
    input.retain_source_input_statistics(statistics)?;
    Ok(input)
}
fn declarations(fixture: &Value, raw: &Value) -> TestResult<SelectedScheduleInput> {
    let mut result = SelectedScheduleInput::default();
    for name in names(fixture, "ScheduleTypeLimits")? {
        let fields = member(member(raw, "ScheduleTypeLimits")?, &name)?;
        // All three actual declarations omit unit_type, retaining Native Invalid.
        if alpha(fields, "unit_type")?.is_some() {
            return Err(invalid("cohort type unit provenance changed").into());
        }
        result.types.push(SourceScheduleTypeInput {
            object_name: name.clone(), name: name.to_ascii_uppercase(),
            lower: number(fields, "lower_limit_value")?, upper: number(fields, "upper_limit_value")?,
            numeric_type: alpha(fields, "numeric_type")?, unit: ScheduleUnitType::Invalid,
        });
    }
    for name in names(fixture, "Schedule:Constant")? {
        let fields = member(member(raw, "Schedule:Constant")?, &name)?;
        result.constants.push(SourceConstantScheduleInput {
            object_name: name.clone(), name: name.to_ascii_uppercase(),
            type_name: alpha(fields, "schedule_type_limits_name")?,
            value: number(fields, "hourly_value")?.ok_or_else(|| invalid("actual constant value"))?,
        });
    }
    Ok(result)
}

#[derive(Default)]
struct MemoryAudit { bytes: Vec<u8> }
impl ScheduleAuditFileHost for MemoryAudit {
    fn audit_path(&self) -> &Path { Path::new("test-only-compact.audit") }
    fn ensure_audit_open(&mut self) -> io::Result<&mut dyn io::Write> { Ok(&mut self.bytes) }
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
        let declarations = declarations(&fixture, raw)?;
        let mut manager = SelectedScheduleManager::default();
        manager.init_constant_state(0)?;
        manager.process_schedule_input(&mut input, &declarations, steps, &mut MemoryAudit::default())?;
        let native_declarations = array(member(cohort, "native_declarations")?)?;
        let mut native_names = Vec::new();
        for declaration in native_declarations {
            let declaration = array(declaration)?;
            if declaration.len() != 2 { return Err(invalid("Native dictionary pair width").into()); }
            native_names.push(string(&declaration[1])?.to_owned());
        }
        assert_eq!(native_names.len(), 5);
        let actual_names = manager.schedules.iter().skip(2).map(|s| s.name.clone()).collect::<Vec<_>>();
        assert_eq!(actual_names, native_names, "{cohort_name}: actual declaration allocation order");
        let mut indices = Vec::new();
        for (ordinal, name) in native_names.iter().enumerate() {
            let index = manager.get_source_schedule(name)?.ok_or_else(|| invalid("actual schedule lookup"))?;
            assert_eq!(index.0, ordinal + 2, "{cohort_name}: real manager index");
            indices.push(index);
        }
        let rows = array(member(cohort, "rows")?)?;
        assert_eq!(rows.len(), 96);
        for (ordinal, row) in rows.iter().enumerate() {
            let row = array(row)?;
            if row.len() != 2 { return Err(invalid("Native cohort row width").into()); }
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
            let (end_minute, fraction) = string(&tuple[6])?.trim().split_once('.')
                .ok_or_else(|| invalid("Native end-minute token"))?;
            assert!(fraction.bytes().all(|c| c == b'0'));
            let end_minute: i32 = end_minute.parse()?;
            let minutes_per_step = 60 / i32::try_from(steps)?;
            assert_eq!(end_minute % minutes_per_step, 0);
            let clock = WeatherGlobalState {
                hour_of_day: hour, time_step: end_minute / minutes_per_step,
                time_steps_in_hour: i32::try_from(steps)?, ..WeatherGlobalState::default()
            };
            let environment = WeatherEnvironmentState {
                day_of_year_schedule: crate::weather::raw::source_ordinal_day(month, day, 1)?,
                day_of_week: 3, holiday_index: 0, dst_indicator: dst,
                ..WeatherEnvironmentState::default()
            };
            manager.update_source_schedule_values(&clock, &environment)?;
            for ((index, expected), name) in indices.iter().zip(values).zip(&native_names) {
                let expected: f64 = string(expected)?.parse()?;
                let actual = manager.source_current_schedule_value(*index)?;
                assert_eq!(actual.to_bits(), expected.to_bits(),
                    "{cohort_name}: {name}, Native tuple {ordinal}: {tuple:?}");
            }
        }
    }
    Ok(())
}
